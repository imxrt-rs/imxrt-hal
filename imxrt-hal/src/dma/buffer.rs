//! DMA memory buffers
//!
//! Provides a safe abstraction over the two support DMA memory buffers:
//!
//! - A normal, statically-allocated array, which we call [`Linear`](struct.Linear.html)
//! - A circular buffer, called [`Circular`](struct.Circular.html)

use super::Element;

use as_slice::{AsMutSlice, AsSlice};
use core::cell::UnsafeCell;
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

/// A dedicated DMA memory buffer for transfers and receives
///
/// `Buffer`s may be statically allocated. They are "owned" by a
/// DMA memory adapter. The ownership is enforced at runtime.
/// `Buffer`s should store arrays of `u8`, `u16`, or `u32` elements.
///
/// ```
/// use imxrt_hal::dma;
/// static UART2_DMA_RX: dma::Buffer<[u8; 256]> = dma::Buffer::new([0; 256]);
/// ```
///
/// DMA memory adapters may enforce additional size or alignment requirements on the
/// statically-allocated buffers. See the adapter's documentation for details.
#[repr(C)] // Need guaranteed layout for checking memory aligmnent, required by circular buffer
pub struct Buffer<B> {
    /// A mutable array that will be used by both the hardware DMA channel
    /// and the user.
    memory: UnsafeCell<B>,
    /// `true` if this buffer has been taken, else `false`
    taken: AtomicBool,
}

// Its safe to allocate `Buffer` as an immutable static. The two `Buffer` adapters
// will guarantee exclusive ownership of the mutable memory by setting the `taken`
// flag. This memory will be shared with the DMA hardware.
unsafe impl<B> Sync for Buffer<B> {}

impl<B> Buffer<B> {
    /// Create a buffer that wraps the provided memory
    ///
    /// May be used to allocate a `static` buffer.
    ///
    /// ```
    /// use imxrt_hal::dma;
    /// static UART2_DMA_RX: dma::Buffer<[u8; 256]> = dma::Buffer::new([0; 256]);
    /// ```
    pub const fn new(memory: B) -> Self {
        Buffer {
            memory: UnsafeCell::new(memory),
            taken: AtomicBool::new(false),
        }
    }
}

/// A linear DMA buffer
///
/// The DMA controller interprets the memory as a normal array. Use [`as_elements()`](struct.Linear.html#method.as_elements)
/// to read from the buffer, or [`as_mut_elements()`](struct.Linear.html#method.as_mut_elements) to read and
/// write from the buffer.
///
/// Use [`set_transfer_len()`](struct.Linear.html#method.set_transfer_len) to specify how many elements
/// in this buffer should be used to satisfy a DMA transfer. By default, the transfer length is equal to
/// the size of the underlying buffer. If your DMA transfer is transferring more, or fewer, elements than
/// expected, ensure that you're calling `set_transfer_len()`.
///
/// The `Linear` adapter will "own" the [`Buffer`](struct.Buffer.html) provided on construction. However, when
/// it's dropped, `Linear` **will not** release ownership. Either keep the object alive, or use the
/// [`new_unchecked()`](struct.Linear.html#method.new_unchecked) method to construct a new `Linear` adapter over
/// the same buffer.
///
/// ```
/// use imxrt_hal::dma;
///
/// static DMA1_BUFFER: dma::Buffer<[u8; 256]> = dma::Buffer::new([0; 256]);
///
/// let mut linear = dma::Linear::new(&DMA1_BUFFER).unwrap();
/// // DMA1_BUFFER is owned by linear. If we try to use it again,
/// // it returns None.
/// assert!(dma::Linear::new(&DMA1_BUFFER).is_none());
///
/// // Fill the first 6 elements, and mark them for transfer
/// linear.as_mut_elements()[..6].copy_from_slice(&[1, 2, 3, 4, 5, 6]);
/// linear.set_transfer_len(6);
/// ```
#[derive(Debug)]
pub struct Linear<E> {
    /// Pointer to array
    ///
    /// Will have static lifetime when using the safe interfaces
    ptr: *mut E,
    /// Length of the static memory buffer
    len: usize,
    /// Usable transfer elements
    ///
    /// User will set this to indicate how many elements should be transferred
    /// into / from this linear memory.
    usable: usize,
}

impl<E> Linear<E>
where
    E: Element,
{
    /// Create a new `Linear` DMA buffer that takes ownership of the memory wrapped
    /// by `buffer`
    ///
    /// If the constructor has exclusive ownership of `buffer`, returns `Some(Linear)`.
    /// Returns `None` if the `buffer` is already owned.
    pub fn new<B>(buffer: &'static Buffer<B>) -> Option<Self>
    where
        B: AsMutSlice<Element = E>,
    {
        let taken = buffer.taken.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            unsafe { Some(Self::new_unchecked(buffer)) }
        }
    }

    /// Create a `Linear` DMA buffer without checking the ownership of the
    /// supplied `buffer`
    ///
    /// # Safety
    ///
    /// Using this method may result in two, mutable references to static memory,
    /// which may cause memory unsafey. Caller must ensure that the provided `buffer`
    /// is no longer owned.
    pub unsafe fn new_unchecked<B>(buffer: &'static Buffer<B>) -> Self
    where
        B: AsMutSlice<Element = E>,
    {
        let memory: &'static mut _ = &mut *buffer.memory.get();
        Self::from_raw(memory)
    }

    /// Create a `Linear` DMA buffer without any concern to ownership or memory
    /// lifetime
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the lifetime of `raw` is greater than the
    /// lifetime of all DMA transfers that use the memory. The caller must guarantee
    /// that there are no other mutable references to this memory.
    ///
    /// ```
    /// use imxrt_hal::dma;
    ///
    /// let mut my_buffer: [u32; 128] = [0; 128];
    /// // my_buffer is stack-allocated, so we need to ensure that the lifetime of
    /// // our linear memory doesn't outlive my_buffer
    /// let linear = unsafe { dma::Linear::from_raw(&mut my_buffer) };
    /// ```
    pub unsafe fn from_raw<B>(raw: &mut B) -> Self
    where
        B: AsMutSlice<Element = E>,
    {
        let ptr = raw.as_mut_slice().as_mut_ptr();
        let len = raw.as_mut_slice().len();
        Linear {
            ptr,
            len,
            usable: len,
        }
    }

    /// Returns a slice to the elements in the linear buffer
    ///
    /// The slice's length is the whole backing buffer, not the length specified
    /// in `set_transfer_len()`.
    pub fn as_elements(&self) -> &[E] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Returns a mutable slice to the elements in the linear buffer
    ///
    /// The slice's length is the whole backing buffer, not the length specified
    /// in `set_transfer_len()`.
    pub fn as_mut_elements(&mut self) -> &mut [E] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    /// Set the number of elements that will be used in a DMA transfer
    ///
    /// The transfer len specifies how many elements, starting from the front of the
    /// linear memory, will be sent during a DMA transfer. Or, it indicates how many
    /// elements should be received from a DMA transfer.
    ///
    /// `len` is capped at the maximum size of the backing buffer.
    pub fn set_transfer_len(&mut self, len: usize) {
        self.usable = len.min(self.len);
    }
}

impl<E: Element> AsRef<[E]> for Linear<E> {
    fn as_ref(&self) -> &[E] {
        self.as_elements()
    }
}

impl<E: Element> AsMut<[E]> for Linear<E> {
    fn as_mut(&mut self) -> &mut [E] {
        self.as_mut_elements()
    }
}

impl<E: Element> AsSlice for Linear<E> {
    type Element = E;
    fn as_slice(&self) -> &[E] {
        self.as_elements()
    }
}

impl<E: Element> AsMutSlice for Linear<E> {
    fn as_mut_slice(&mut self) -> &mut [E] {
        self.as_mut_elements()
    }
}

/// A circular DMA buffer
///
/// `Circular` provides a [`push()`](struct.Circular.html#method.push) and [`pop()`](struct.Circular.html#method.pop)
/// interface to manipulate the backing memory. Unlike a [`Linear`](struct.Linear.html) adapter, a `Circular` may be accessed
/// while a transfer is in progress. For example, you may continue to `push()` elements into a `Circular` while a DMA transfer
/// is reading from the previously-pushed values. The DMA controller will interpret the memory as a circular buffer, and it will
/// wrap around when reading / writing elements to the memory, just as `push()` and `pop()` wrap around the buffer.
///
/// `Circular` has two requirements:
///
/// - the size of the backing [`Buffer`](struct.Buffer.html) is a power of two
/// - the *alignment* of the `Buffer` is a multiple of the *size* of the `Buffer`
///
/// If you don't hold these two requirements, you will fail to construct a `Circular`. We enforce the requirements even through
/// the `unsafe` interface.
///
/// To enforce the alignment requirement, create a newtype struct that specifies an alignment. See the example for more guidance.
///
/// The capacity of a `Circular` buffer is one less than the size of the backing memory. For example, if a `Circular` is backed
/// by a 512-element buffer, the capacity is 511.
///
/// # Using `Circular` as a DMA source
///
/// - `push()` your elements into the buffer (A)
/// - hand-off your `Circular` instance to another DMA object
/// - (optional) during the transfer, continue to `push()` elements into the buffer
/// - when the transfer completes, you will be able to overwrite the elements supplied during (A)
///
/// # Using `Circular` as a DMA destination
///
/// - [`reserve()`](struct.Circular.html#method.reserve) a number of elements to hold the incoming elements
/// - hand-off your `Circular` instance to another DMA object
/// - when the transfer completes, you will be able to `pop()` or [`drain()`](struct.Circular.html#method.drain) the received elements
///
/// The `Circular` adapter will "own" the `Buffer` provided on construction. However, when
/// it's dropped, `Circular` **will not** release ownership. Either keep the object alive, or use the
/// [`new_unchecked()`](struct.Circular.html#method.new_unchecked) method to construct a new `Circular` adapter over
/// the same buffer.
///
/// # Example
///
/// ```
///
/// use imxrt_hal::dma;
///
/// // A newtype to enforce the required alignment
/// #[repr(align(512))]
/// struct Align512(dma::Buffer<[u16; 512]>);
///
/// static BUFFER: Align512 = Align512(dma::Buffer::new([0; 512]));
///
/// let mut circular = dma::Circular::new(&BUFFER.0).unwrap();
/// // BUFFER is taken and cannot be used again
/// assert_eq!(dma::Circular::new(&BUFFER.0).unwrap_err(), dma::CircularError::BufferTaken);
///
/// // The maximum number of elements is one less than the size of the backing
/// // memory.
/// assert_eq!(circular.capacity(), 511);
///
/// circular.push(1);
/// circular.push(2);
/// circular.push(3);
/// assert_eq!(circular.len(), 3);
///
/// assert_eq!(circular.pop(), Some(1));
/// assert_eq!(circular.pop(), Some(2));
/// assert!(!circular.is_empty());
///
/// assert_eq!(circular.pop(), Some(3));
/// assert_eq!(circular.pop(), None);
/// assert!(circular.is_empty());
/// ```
///
/// If the underlying buffer size is *not* a power of two, we cannot create a
/// circular DMA queue:
///
/// ```
/// use imxrt_hal::dma;
///
/// #[repr(align(32))]
/// struct Align32(dma::Buffer<[u16; 30]>);
/// static BUFFER: Align32 = Align32(dma::Buffer::new([0; 30]));
///
/// let err = dma::Circular::new(&BUFFER.0).expect_err("30 is not a power of two");
/// assert_eq!(err, dma::CircularError::NotPowerOfTwo);
/// ```
#[derive(Debug)]
pub struct Circular<E> {
    /// Pointer to memory buffer
    ///
    /// Lifetime is static when using the safe interface
    ptr: *mut E,
    /// Total capacity of the buffer (length of the array)
    ///
    /// Will be a power of two
    cap: usize,
    /// Read position
    read: usize,
    /// Write position
    write: usize,
    /// Reserved elements for a transfer
    ///
    /// When used as a source, `reserved` denotes how many elements are involved in the
    /// transfer. It's a snapshot of `len()` at the point of starting the transfer.
    ///
    /// When used as a destination, `reserved` denotes how may elements will be read into
    /// the circular queue.
    reserved: usize,
}

/// Possible errors when creating a circular buffer
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CircularError {
    /// The size of the memory is not a power of two
    NotPowerOfTwo,
    /// The buffer is taken, likely used in another DMA buffer
    BufferTaken,
    /// The alignment of the buffer must be a multiple of the buffer's
    /// size
    IncorrectAlignment,
}

impl<E: Element> Circular<E> {
    /// Creates a new circular DMA buffer using the memory supplied by `buffer`
    ///
    /// If the `buffer` is already taken, returns `Err(CircularError::BufferTaken)`.
    /// If the size of the buffer is *not* a power of two, returns `Err(CircularError::NotPowerOfTwo)`.
    /// If the buffer is not aligned to a multiple of the size, returns `Err(CircularError::IncorrectAlignment)`.
    pub fn new<B>(buffer: &'static Buffer<B>) -> Result<Self, CircularError>
    where
        B: AsMutSlice<Element = E>,
    {
        let taken = buffer.taken.swap(true, Ordering::SeqCst);
        if taken {
            Err(CircularError::BufferTaken)
        } else {
            // Safety: it's not taken
            unsafe { Self::new_unchecked(buffer) }.or_else(|err| {
                buffer.taken.store(false, Ordering::SeqCst);
                Err(err)
            })
        }
    }

    /// Creates a new circular DMA buffer using the memory supplied by `buffer`, but do not
    /// check for buffer ownership.
    ///
    /// If the size of the buffer is *not* a power of two, returns `Err(CircularError::NotPowerOfTwo)`.
    /// If the buffer is not aligned to a multiple of the size, returns `Err(CircularError::IncorrectAlignment)`.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the `buffer` is not in use anywhere else. Otherwise, there will be
    /// more than one owner of mutable memory.
    pub unsafe fn new_unchecked<B>(buffer: &'static Buffer<B>) -> Result<Self, CircularError>
    where
        B: AsMutSlice<Element = E>,
    {
        Self::from_raw(&mut *buffer.memory.get())
    }

    /// Creates a new circular DMA buffer from arbitrary memory
    ///
    /// If the size of the buffer is *not* a power of two, returns `Err(CircularError::NotPowerOfTwo)`.
    /// If the buffer is not aligned to a multiple of the size, returns `Err(CircularError::IncorrectAlignment)`.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the lifetime of `raw` is greater than all the DMA transfers that use
    /// the memory. Caller must ensure that there are no other mutable references to this memory.
    ///
    /// ```
    /// use imxrt_hal::dma;
    ///
    /// #[repr(align(64))]
    /// struct Align64([u8; 64]);
    ///
    /// let mut my_memory = Align64([0; 64]);
    ///
    /// // my_memory is stack-allocated, so we need to ensure that the `Circular` doesn't
    /// // outlive my_memory.
    /// let mut circular = unsafe { dma::Circular::from_raw(&mut my_memory.0).unwrap() };
    /// ```
    pub unsafe fn from_raw<B>(raw: &mut B) -> Result<Self, CircularError>
    where
        B: AsMutSlice<Element = E>,
    {
        let cap = raw.as_mut_slice().len();
        let ptr = raw.as_mut_slice().as_mut_ptr();
        if !cap.is_power_of_two() {
            Err(CircularError::NotPowerOfTwo)
        } else if (ptr as usize) % cap != 0 {
            Err(CircularError::IncorrectAlignment)
        } else {
            Ok(Circular {
                ptr,
                cap,
                read: 0,
                write: 0,
                reserved: 0,
            })
        }
    }

    /// Returns the number of readable elements in the queue
    pub fn len(&self) -> usize {
        self.write.wrapping_sub(self.read) & (self.cap - 1)
    }

    /// Returns `true` if the queue has no readable elements
    pub fn is_empty(&self) -> bool {
        self.write == self.read
    }

    /// Returns the number of elements the circular queue can hold
    ///
    /// The capacity is always one less than the number of elements in the backing
    /// buffer.
    pub fn capacity(&self) -> usize {
        self.cap - 1
    }

    /// Pushes an element into the circular queue
    ///
    /// Returns `true` if the element was enqueued, or `false`
    /// if there wasn't enough space
    pub fn push(&mut self, element: E) -> bool {
        if self.len() == (self.cap - 1) {
            false
        } else {
            // Safety: HAL implementers enforce static lifetime; or, users ensure
            // the pointer is valid.
            unsafe {
                ptr::write(self.write_ptr(), element);
            }
            self.mark_written(1);
            true
        }
    }

    /// Inserts elements from `iter` into the circular buffer, returning the number
    /// elements inserted into the buffer
    ///
    /// `insert` may *not* insert all the elements into the buffer.
    ///
    /// ```
    /// use imxrt_hal::dma;
    /// #[repr(align(32))]
    /// struct Align32(dma::Buffer<[u16; 32]>);
    ///
    /// static BUFFER: Align32 = Align32(dma::Buffer::new([0; 32]));
    ///
    /// let mut circular = dma::Circular::new(&BUFFER.0).unwrap();
    /// assert_eq!(circular.insert(0..30), 30);
    /// assert_eq!(circular.insert(31..60), 1);
    /// ```
    pub fn insert<I>(&mut self, iter: I) -> usize
    where
        I: IntoIterator<Item = E>,
    {
        let mut inserts = 0;
        for elem in iter {
            if self.push(elem) {
                inserts += 1;
            } else {
                break;
            }
        }
        inserts
    }

    /// Peeks at the next element in the queue
    ///
    /// Returns `None` if there are no elements to read.
    pub fn peek(&self) -> Option<E> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { ptr::read(self.read_ptr()) })
        }
    }

    /// Remove the next element from the queue
    pub fn pop(&mut self) -> Option<E> {
        self.peek().and_then(|elem| {
            self.mark_read(1);
            Some(elem)
        })
    }

    /// Returns an iterator that can drain the readable contents from
    /// the circular queue
    ///
    /// Each iteration calls `pop()`, returning the next readable element
    /// in the queue, until the readable elements are exhausted.
    pub fn drain(&mut self) -> Drain<E> {
        Drain(self)
    }

    /// Reserves `reservation` number of elements to be used as a DMA transfer destination
    ///
    /// Use `reserve()` when you want to receive data into the circular buffer. Once the transfer
    /// completes, the `reservation` number of elements will be readable.
    ///
    /// `reservation` is capped at the capacity of the circular buffer.
    pub fn reserve(&mut self, reservation: usize) {
        self.reserved = reservation.min(self.capacity());
    }

    /// Returns the pointer to the start of the readable queue memory
    fn read_ptr(&self) -> *const E {
        unsafe { self.ptr.add(self.read) }
    }

    /// Returns the pointer to the start of the writeable queue memory
    fn write_ptr(&mut self) -> *mut E {
        unsafe { self.ptr.add(self.write) }
    }

    /// Mark `size` elements as read
    ///
    /// Equivalent to calling `pop()` `size` times, and dropping
    /// the elements.
    fn mark_read(&mut self, size: usize) {
        self.read = (self.read + size) & (self.cap - 1);
    }

    /// Mark `size` elements as written
    ///
    /// Equivalent to calling `push()` `size` times.
    fn mark_written(&mut self, size: usize) {
        self.write = (self.write + size) & (self.cap - 1)
    }
}

/// An iterator that will drain the contents from
/// a circular DMA buffer.
///
/// See the [`drain()`] method for details.
pub struct Drain<'a, E>(&'a mut Circular<E>);

impl<'a, E: Element> Iterator for Drain<'a, E> {
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}

impl<'a, E: Element> ExactSizeIterator for Drain<'a, E> {}

/// A buffer that can be used as the source of a DMA transfer
pub trait Source<E: Element>: private::Sealed {
    /// Prepare the buffer to be used as a source of a DMA transfer, returning the
    /// number of elements available to transfer
    ///
    /// The implementation should tell the provided `channel` how to interact
    /// with this buffer's memory.
    fn set_source(&mut self, channel: &mut super::Channel) -> usize;
    /// Invoked when the DMA transfer is complete
    ///
    /// Use this to perform any final state transformations before hand-off to
    /// the user.
    fn complete_source(&mut self);
}

/// A buffer that can be used as the destination of a DMA transfer
pub trait Destination<E: Element>: private::Sealed {
    /// Prepare the buffer to be used as the destination for a DMA transfer, returning
    /// the number of elements available to transfer
    ///
    /// The implementation should tell the provided `channel` how to interact
    /// with this buffer's memory.
    fn set_destination(&mut self, channel: &mut super::Channel) -> usize;
    /// Invoked when the DMA transfer is complete
    ///
    /// Use this to perform any final state transformations before hand-off to
    /// the user.
    fn complete_destination(&mut self);
}

mod private {
    pub trait Sealed {}

    use super::{Circular, Linear};
    impl<E> Sealed for Linear<E> {}
    impl<E> Sealed for Circular<E> {}
}

//
// Linear Sources and Destinations
//

impl<E: Element> Source<E> for Linear<E> {
    fn set_source(&mut self, channel: &mut super::Channel) -> usize {
        unsafe {
            channel.set_source_buffer(&self.as_elements()[..self.usable]);
        }
        self.usable
    }
    fn complete_source(&mut self) {}
}

impl<E: Element> Destination<E> for Linear<E> {
    fn set_destination(&mut self, channel: &mut super::Channel) -> usize {
        let offset = self.usable;
        unsafe {
            channel.set_destination_buffer(&mut self.as_mut_elements()[..offset]);
        }
        self.usable
    }
    fn complete_destination(&mut self) {}
}

//
// Circular Sources and Destinations
//

impl<E: Element> Source<E> for Circular<E> {
    fn set_source(&mut self, channel: &mut super::Channel) -> usize {
        unsafe { channel.set_source_circular(self.read_ptr(), self.cap) };
        self.reserved = self.len();
        self.reserved
    }
    fn complete_source(&mut self) {
        self.mark_read(self.reserved);
    }
}

impl<E: Element> Destination<E> for Circular<E> {
    fn set_destination(&mut self, channel: &mut super::Channel) -> usize {
        unsafe { channel.set_destination_circular(self.write_ptr(), self.cap) }
        self.reserved
    }
    fn complete_destination(&mut self) {
        self.mark_written(self.reserved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Alignment is required by the hardware; we don't need it in tests
    ///
    /// # Safety
    ///
    /// Same guarantees as `from_raw()`, but no runtime checks for size or alignment.
    unsafe fn from_raw_unaligned<B, E>(raw: &mut B) -> Circular<E>
    where
        B: AsMutSlice<Element = E>,
    {
        let cap = raw.as_mut_slice().len();
        let ptr = raw.as_mut_slice().as_mut_ptr();
        Circular {
            ptr,
            cap,
            read: 0,
            write: 0,
            reserved: 0,
        }
    }

    #[test]
    fn circular_simulate_transfer() {
        let mut memory = [0; 8];
        let mut circular: Circular<u8> = unsafe { from_raw_unaligned(&mut memory) };

        // User puts some elements into the buffer
        assert!(circular.push(5));
        assert!(circular.push(6));
        assert!(circular.push(3));

        // User passes it into another DMA type, which marks the number of elements
        // reserved.
        circular.reserved = circular.len();

        // We can push 4 more elements.
        for i in 0..4 {
            assert!(circular.push(i));
        }

        // Can't push more until the transfer is complete
        assert!(!circular.push(0xff));

        // Transfer completes! DMA module marks is as read
        circular.mark_read(circular.reserved);

        // We can push 3 elements
        for i in 0..3 {
            assert!(circular.push(i));
        }

        // All out of room
        assert!(!circular.push(0xdd));
    }

    #[test]
    fn circular_simulate_receive() {
        let mut memory: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut circular: Circular<u8> = unsafe { from_raw_unaligned(&mut memory) };

        // User reserves some number of elements for the transfer
        circular.reserve(5);

        // User passes it into another DMA type. This doesn't do anything.
        // There's nothing to read.
        assert!(circular.pop().is_none());

        // Transfer completes
        circular.mark_written(circular.reserved);

        // User can read values.
        let mut expected = 1;
        for actual in circular.drain() {
            assert_eq!(expected, actual);
            expected += 1;
        }
        assert_eq!(expected, 6);
    }
}
