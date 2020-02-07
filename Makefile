all: patch svd2rust

.PHONY: patch svd2rust form check clean-rs clean-patch clean-html clean
.PRECIOUS: svd/%.svd .deps/%.d

SHELL := /usr/bin/env bash

CRATES ?= imxrt1011 imxrt1015 imxrt1021 imxrt1051 imxrt1052 imxrt1061 imxrt1062 imxrt1064 

# All yaml files in devices/ will be used to patch an SVD
YAMLS := $(foreach crate, $(CRATES), \
	       $(wildcard devices/$(crate)*.yaml))

# Each yaml file in devices/ exactly name-matches an SVD file in svd/
PATCHED_SVDS := $(patsubst devices/%.yaml, svd/%.svd.patched, $(YAMLS))
FORMATTED_SVDS := $(patsubst devices/%.yaml, svd/%.svd.formatted, $(YAMLS))

# Each device will lead to a crate/src/device/mod.rs file
RUST_SRCS := $(foreach crate, $(CRATES), \
               $(patsubst devices/$(crate)%.yaml, \
                          $(crate)/src/lib.rs, \
                          $(wildcard devices/$(crate)*.yaml)))
RUST_DIRS := $(foreach crate, $(CRATES), \
               $(patsubst devices/$(crate)%.yaml, \
                          $(crate)/src/$(crate)%/, \
                          $(wildcard devices/$(crate)*.yaml)))
FORM_SRCS := $(foreach crate, $(CRATES), \
               $(patsubst devices/$(crate)%.yaml, \
                          $(crate)/src/.form, \
                          $(wildcard devices/$(crate)*.yaml)))
CHECK_SRCS := $(foreach crate, $(CRATES), \
               $(patsubst devices/$(crate)%.yaml, \
                          $(crate)/src/.check, \
                          $(wildcard devices/$(crate)*.yaml)))

# Turn a devices/device.yaml and svd/device.svd into svd/device.svd.patched
svd/%.svd.patched: devices/%.yaml svd/%.svd .deps/%.d
	svd patch $<

svd/%.svd.formatted: svd/%.svd.patched
	xmllint $< --format -o $@

define crate_template
$(1)/src/lib.rs: svd/$(1).svd.patched
	cd $(1); svd2rust -i ../$$<
	cd $(1); mv lib.rs src/
	rustfmt --config-path="rustfmt.toml" $$(@D)/lib.rs
	
$(1)/src/.form: $(1)/src/lib.rs
	form -i $$< -o $$(@D)
	rm $$<
	mv $$(@D)/lib.rs $$<
	rustfmt --config-path="rustfmt.toml" $$<
	touch $$@

$(1)/src/.check: $(1)/src/lib.rs
	cd $(1) && cargo check --target-dir ../target/check/ --features rt
	touch $$@

endef

$(foreach crate,$(CRATES),$(eval $(call crate_template,$(crate))))

patch: $(PATCHED_SVDS)

svd2rust: $(RUST_SRCS)

form: $(FORM_SRCS)

svdformat: $(FORMATTED_SVDS)

check: $(CHECK_SRCS)

html/index.html: $(PATCHED_SVDS)
	@mkdir -p html
	python3 scripts/makehtml.py html/ svd/stm32*.svd.patched

html: html/index.html

clean-rs:
	rm -rf $(RUST_DIRS)
	rm -f */src/generic.rs

clean-patch:
	rm -f $(PATCHED_SVDS)
	rm -f $(FORMATTED_SVDS)

clean-html:
	rm -rf html

clean: clean-rs clean-patch clean-html
	rm -rf .deps

# As alternative to `pip install --user svdtools`:
# run `make venv update-venv` and `source venv/bin/activate'
venv:
	python3 -m venv venv

update-venv:
	venv/bin/pip install -U pip
	venv/bin/pip install -U -r requirements.txt

# Generate dependencies for each device YAML
.deps/%.d: devices/%.yaml
	@mkdir -p .deps
	python3 scripts/makedeps.py $< > $@

-include .deps/*
