#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Instrumentation Trace Macrocell
//!
//! Used by: armv7em, armv7m

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Stimulus Port Register 0
pub mod STIM0 {}

/// Stimulus Port Register 1
pub mod STIM1 {}

/// Stimulus Port Register 2
pub mod STIM2 {}

/// Stimulus Port Register 3
pub mod STIM3 {}

/// Stimulus Port Register 4
pub mod STIM4 {}

/// Stimulus Port Register 5
pub mod STIM5 {}

/// Stimulus Port Register 6
pub mod STIM6 {}

/// Stimulus Port Register 7
pub mod STIM7 {}

/// Stimulus Port Register 8
pub mod STIM8 {}

/// Stimulus Port Register 9
pub mod STIM9 {}

/// Stimulus Port Register 10
pub mod STIM10 {}

/// Stimulus Port Register 11
pub mod STIM11 {}

/// Stimulus Port Register 12
pub mod STIM12 {}

/// Stimulus Port Register 13
pub mod STIM13 {}

/// Stimulus Port Register 14
pub mod STIM14 {}

/// Stimulus Port Register 15
pub mod STIM15 {}

/// Stimulus Port Register 16
pub mod STIM16 {}

/// Stimulus Port Register 17
pub mod STIM17 {}

/// Stimulus Port Register 18
pub mod STIM18 {}

/// Stimulus Port Register 19
pub mod STIM19 {}

/// Stimulus Port Register 20
pub mod STIM20 {}

/// Stimulus Port Register 21
pub mod STIM21 {}

/// Stimulus Port Register 22
pub mod STIM22 {}

/// Stimulus Port Register 23
pub mod STIM23 {}

/// Stimulus Port Register 24
pub mod STIM24 {}

/// Stimulus Port Register 25
pub mod STIM25 {}

/// Stimulus Port Register 26
pub mod STIM26 {}

/// Stimulus Port Register 27
pub mod STIM27 {}

/// Stimulus Port Register 28
pub mod STIM28 {}

/// Stimulus Port Register 29
pub mod STIM29 {}

/// Stimulus Port Register 30
pub mod STIM30 {}

/// Stimulus Port Register 31
pub mod STIM31 {}

/// Stimulus Port Register 32
pub mod STIM32 {}

/// Stimulus Port Register 33
pub mod STIM33 {}

/// Stimulus Port Register 34
pub mod STIM34 {}

/// Stimulus Port Register 35
pub mod STIM35 {}

/// Stimulus Port Register 36
pub mod STIM36 {}

/// Stimulus Port Register 37
pub mod STIM37 {}

/// Stimulus Port Register 38
pub mod STIM38 {}

/// Stimulus Port Register 39
pub mod STIM39 {}

/// Stimulus Port Register 40
pub mod STIM40 {}

/// Stimulus Port Register 41
pub mod STIM41 {}

/// Stimulus Port Register 42
pub mod STIM42 {}

/// Stimulus Port Register 43
pub mod STIM43 {}

/// Stimulus Port Register 44
pub mod STIM44 {}

/// Stimulus Port Register 45
pub mod STIM45 {}

/// Stimulus Port Register 46
pub mod STIM46 {}

/// Stimulus Port Register 47
pub mod STIM47 {}

/// Stimulus Port Register 48
pub mod STIM48 {}

/// Stimulus Port Register 49
pub mod STIM49 {}

/// Stimulus Port Register 50
pub mod STIM50 {}

/// Stimulus Port Register 51
pub mod STIM51 {}

/// Stimulus Port Register 52
pub mod STIM52 {}

/// Stimulus Port Register 53
pub mod STIM53 {}

/// Stimulus Port Register 54
pub mod STIM54 {}

/// Stimulus Port Register 55
pub mod STIM55 {}

/// Stimulus Port Register 56
pub mod STIM56 {}

/// Stimulus Port Register 57
pub mod STIM57 {}

/// Stimulus Port Register 58
pub mod STIM58 {}

/// Stimulus Port Register 59
pub mod STIM59 {}

/// Stimulus Port Register 60
pub mod STIM60 {}

/// Stimulus Port Register 61
pub mod STIM61 {}

/// Stimulus Port Register 62
pub mod STIM62 {}

/// Stimulus Port Register 63
pub mod STIM63 {}

/// Stimulus Port Register 64
pub mod STIM64 {}

/// Stimulus Port Register 65
pub mod STIM65 {}

/// Stimulus Port Register 66
pub mod STIM66 {}

/// Stimulus Port Register 67
pub mod STIM67 {}

/// Stimulus Port Register 68
pub mod STIM68 {}

/// Stimulus Port Register 69
pub mod STIM69 {}

/// Stimulus Port Register 70
pub mod STIM70 {}

/// Stimulus Port Register 71
pub mod STIM71 {}

/// Stimulus Port Register 72
pub mod STIM72 {}

/// Stimulus Port Register 73
pub mod STIM73 {}

/// Stimulus Port Register 74
pub mod STIM74 {}

/// Stimulus Port Register 75
pub mod STIM75 {}

/// Stimulus Port Register 76
pub mod STIM76 {}

/// Stimulus Port Register 77
pub mod STIM77 {}

/// Stimulus Port Register 78
pub mod STIM78 {}

/// Stimulus Port Register 79
pub mod STIM79 {}

/// Stimulus Port Register 80
pub mod STIM80 {}

/// Stimulus Port Register 81
pub mod STIM81 {}

/// Stimulus Port Register 82
pub mod STIM82 {}

/// Stimulus Port Register 83
pub mod STIM83 {}

/// Stimulus Port Register 84
pub mod STIM84 {}

/// Stimulus Port Register 85
pub mod STIM85 {}

/// Stimulus Port Register 86
pub mod STIM86 {}

/// Stimulus Port Register 87
pub mod STIM87 {}

/// Stimulus Port Register 88
pub mod STIM88 {}

/// Stimulus Port Register 89
pub mod STIM89 {}

/// Stimulus Port Register 90
pub mod STIM90 {}

/// Stimulus Port Register 91
pub mod STIM91 {}

/// Stimulus Port Register 92
pub mod STIM92 {}

/// Stimulus Port Register 93
pub mod STIM93 {}

/// Stimulus Port Register 94
pub mod STIM94 {}

/// Stimulus Port Register 95
pub mod STIM95 {}

/// Stimulus Port Register 96
pub mod STIM96 {}

/// Stimulus Port Register 97
pub mod STIM97 {}

/// Stimulus Port Register 98
pub mod STIM98 {}

/// Stimulus Port Register 99
pub mod STIM99 {}

/// Stimulus Port Register 100
pub mod STIM100 {}

/// Stimulus Port Register 101
pub mod STIM101 {}

/// Stimulus Port Register 102
pub mod STIM102 {}

/// Stimulus Port Register 103
pub mod STIM103 {}

/// Stimulus Port Register 104
pub mod STIM104 {}

/// Stimulus Port Register 105
pub mod STIM105 {}

/// Stimulus Port Register 106
pub mod STIM106 {}

/// Stimulus Port Register 107
pub mod STIM107 {}

/// Stimulus Port Register 108
pub mod STIM108 {}

/// Stimulus Port Register 109
pub mod STIM109 {}

/// Stimulus Port Register 110
pub mod STIM110 {}

/// Stimulus Port Register 111
pub mod STIM111 {}

/// Stimulus Port Register 112
pub mod STIM112 {}

/// Stimulus Port Register 113
pub mod STIM113 {}

/// Stimulus Port Register 114
pub mod STIM114 {}

/// Stimulus Port Register 115
pub mod STIM115 {}

/// Stimulus Port Register 116
pub mod STIM116 {}

/// Stimulus Port Register 117
pub mod STIM117 {}

/// Stimulus Port Register 118
pub mod STIM118 {}

/// Stimulus Port Register 119
pub mod STIM119 {}

/// Stimulus Port Register 120
pub mod STIM120 {}

/// Stimulus Port Register 121
pub mod STIM121 {}

/// Stimulus Port Register 122
pub mod STIM122 {}

/// Stimulus Port Register 123
pub mod STIM123 {}

/// Stimulus Port Register 124
pub mod STIM124 {}

/// Stimulus Port Register 125
pub mod STIM125 {}

/// Stimulus Port Register 126
pub mod STIM126 {}

/// Stimulus Port Register 127
pub mod STIM127 {}

/// Stimulus Port Register 128
pub mod STIM128 {}

/// Stimulus Port Register 129
pub mod STIM129 {}

/// Stimulus Port Register 130
pub mod STIM130 {}

/// Stimulus Port Register 131
pub mod STIM131 {}

/// Stimulus Port Register 132
pub mod STIM132 {}

/// Stimulus Port Register 133
pub mod STIM133 {}

/// Stimulus Port Register 134
pub mod STIM134 {}

/// Stimulus Port Register 135
pub mod STIM135 {}

/// Stimulus Port Register 136
pub mod STIM136 {}

/// Stimulus Port Register 137
pub mod STIM137 {}

/// Stimulus Port Register 138
pub mod STIM138 {}

/// Stimulus Port Register 139
pub mod STIM139 {}

/// Stimulus Port Register 140
pub mod STIM140 {}

/// Stimulus Port Register 141
pub mod STIM141 {}

/// Stimulus Port Register 142
pub mod STIM142 {}

/// Stimulus Port Register 143
pub mod STIM143 {}

/// Stimulus Port Register 144
pub mod STIM144 {}

/// Stimulus Port Register 145
pub mod STIM145 {}

/// Stimulus Port Register 146
pub mod STIM146 {}

/// Stimulus Port Register 147
pub mod STIM147 {}

/// Stimulus Port Register 148
pub mod STIM148 {}

/// Stimulus Port Register 149
pub mod STIM149 {}

/// Stimulus Port Register 150
pub mod STIM150 {}

/// Stimulus Port Register 151
pub mod STIM151 {}

/// Stimulus Port Register 152
pub mod STIM152 {}

/// Stimulus Port Register 153
pub mod STIM153 {}

/// Stimulus Port Register 154
pub mod STIM154 {}

/// Stimulus Port Register 155
pub mod STIM155 {}

/// Stimulus Port Register 156
pub mod STIM156 {}

/// Stimulus Port Register 157
pub mod STIM157 {}

/// Stimulus Port Register 158
pub mod STIM158 {}

/// Stimulus Port Register 159
pub mod STIM159 {}

/// Stimulus Port Register 160
pub mod STIM160 {}

/// Stimulus Port Register 161
pub mod STIM161 {}

/// Stimulus Port Register 162
pub mod STIM162 {}

/// Stimulus Port Register 163
pub mod STIM163 {}

/// Stimulus Port Register 164
pub mod STIM164 {}

/// Stimulus Port Register 165
pub mod STIM165 {}

/// Stimulus Port Register 166
pub mod STIM166 {}

/// Stimulus Port Register 167
pub mod STIM167 {}

/// Stimulus Port Register 168
pub mod STIM168 {}

/// Stimulus Port Register 169
pub mod STIM169 {}

/// Stimulus Port Register 170
pub mod STIM170 {}

/// Stimulus Port Register 171
pub mod STIM171 {}

/// Stimulus Port Register 172
pub mod STIM172 {}

/// Stimulus Port Register 173
pub mod STIM173 {}

/// Stimulus Port Register 174
pub mod STIM174 {}

/// Stimulus Port Register 175
pub mod STIM175 {}

/// Stimulus Port Register 176
pub mod STIM176 {}

/// Stimulus Port Register 177
pub mod STIM177 {}

/// Stimulus Port Register 178
pub mod STIM178 {}

/// Stimulus Port Register 179
pub mod STIM179 {}

/// Stimulus Port Register 180
pub mod STIM180 {}

/// Stimulus Port Register 181
pub mod STIM181 {}

/// Stimulus Port Register 182
pub mod STIM182 {}

/// Stimulus Port Register 183
pub mod STIM183 {}

/// Stimulus Port Register 184
pub mod STIM184 {}

/// Stimulus Port Register 185
pub mod STIM185 {}

/// Stimulus Port Register 186
pub mod STIM186 {}

/// Stimulus Port Register 187
pub mod STIM187 {}

/// Stimulus Port Register 188
pub mod STIM188 {}

/// Stimulus Port Register 189
pub mod STIM189 {}

/// Stimulus Port Register 190
pub mod STIM190 {}

/// Stimulus Port Register 191
pub mod STIM191 {}

/// Stimulus Port Register 192
pub mod STIM192 {}

/// Stimulus Port Register 193
pub mod STIM193 {}

/// Stimulus Port Register 194
pub mod STIM194 {}

/// Stimulus Port Register 195
pub mod STIM195 {}

/// Stimulus Port Register 196
pub mod STIM196 {}

/// Stimulus Port Register 197
pub mod STIM197 {}

/// Stimulus Port Register 198
pub mod STIM198 {}

/// Stimulus Port Register 199
pub mod STIM199 {}

/// Stimulus Port Register 200
pub mod STIM200 {}

/// Stimulus Port Register 201
pub mod STIM201 {}

/// Stimulus Port Register 202
pub mod STIM202 {}

/// Stimulus Port Register 203
pub mod STIM203 {}

/// Stimulus Port Register 204
pub mod STIM204 {}

/// Stimulus Port Register 205
pub mod STIM205 {}

/// Stimulus Port Register 206
pub mod STIM206 {}

/// Stimulus Port Register 207
pub mod STIM207 {}

/// Stimulus Port Register 208
pub mod STIM208 {}

/// Stimulus Port Register 209
pub mod STIM209 {}

/// Stimulus Port Register 210
pub mod STIM210 {}

/// Stimulus Port Register 211
pub mod STIM211 {}

/// Stimulus Port Register 212
pub mod STIM212 {}

/// Stimulus Port Register 213
pub mod STIM213 {}

/// Stimulus Port Register 214
pub mod STIM214 {}

/// Stimulus Port Register 215
pub mod STIM215 {}

/// Stimulus Port Register 216
pub mod STIM216 {}

/// Stimulus Port Register 217
pub mod STIM217 {}

/// Stimulus Port Register 218
pub mod STIM218 {}

/// Stimulus Port Register 219
pub mod STIM219 {}

/// Stimulus Port Register 220
pub mod STIM220 {}

/// Stimulus Port Register 221
pub mod STIM221 {}

/// Stimulus Port Register 222
pub mod STIM222 {}

/// Stimulus Port Register 223
pub mod STIM223 {}

/// Stimulus Port Register 224
pub mod STIM224 {}

/// Stimulus Port Register 225
pub mod STIM225 {}

/// Stimulus Port Register 226
pub mod STIM226 {}

/// Stimulus Port Register 227
pub mod STIM227 {}

/// Stimulus Port Register 228
pub mod STIM228 {}

/// Stimulus Port Register 229
pub mod STIM229 {}

/// Stimulus Port Register 230
pub mod STIM230 {}

/// Stimulus Port Register 231
pub mod STIM231 {}

/// Stimulus Port Register 232
pub mod STIM232 {}

/// Stimulus Port Register 233
pub mod STIM233 {}

/// Stimulus Port Register 234
pub mod STIM234 {}

/// Stimulus Port Register 235
pub mod STIM235 {}

/// Stimulus Port Register 236
pub mod STIM236 {}

/// Stimulus Port Register 237
pub mod STIM237 {}

/// Stimulus Port Register 238
pub mod STIM238 {}

/// Stimulus Port Register 239
pub mod STIM239 {}

/// Stimulus Port Register 240
pub mod STIM240 {}

/// Stimulus Port Register 241
pub mod STIM241 {}

/// Stimulus Port Register 242
pub mod STIM242 {}

/// Stimulus Port Register 243
pub mod STIM243 {}

/// Stimulus Port Register 244
pub mod STIM244 {}

/// Stimulus Port Register 245
pub mod STIM245 {}

/// Stimulus Port Register 246
pub mod STIM246 {}

/// Stimulus Port Register 247
pub mod STIM247 {}

/// Stimulus Port Register 248
pub mod STIM248 {}

/// Stimulus Port Register 249
pub mod STIM249 {}

/// Stimulus Port Register 250
pub mod STIM250 {}

/// Stimulus Port Register 251
pub mod STIM251 {}

/// Stimulus Port Register 252
pub mod STIM252 {}

/// Stimulus Port Register 253
pub mod STIM253 {}

/// Stimulus Port Register 254
pub mod STIM254 {}

/// Stimulus Port Register 255
pub mod STIM255 {}

/// Trace Enable Register 0
pub mod TER0 {}

/// Trace Enable Register 1
pub mod TER1 {}

/// Trace Enable Register 2
pub mod TER2 {}

/// Trace Enable Register 3
pub mod TER3 {}

/// Trace Enable Register 4
pub mod TER4 {}

/// Trace Enable Register 5
pub mod TER5 {}

/// Trace Enable Register 6
pub mod TER6 {}

/// Trace Enable Register 7
pub mod TER7 {}

/// Trace Privilege Register
pub mod TPR {}

/// Trace Control Register
pub mod TCR {}

/// Lock Status Register
pub mod LSR {}

/// Lock Access Register
pub mod LAR {}
pub struct RegisterBlock {
    /// Stimulus Port Register 0
    pub STIM0: RWRegister<u32>,

    /// Stimulus Port Register 1
    pub STIM1: RWRegister<u32>,

    /// Stimulus Port Register 2
    pub STIM2: RWRegister<u32>,

    /// Stimulus Port Register 3
    pub STIM3: RWRegister<u32>,

    /// Stimulus Port Register 4
    pub STIM4: RWRegister<u32>,

    /// Stimulus Port Register 5
    pub STIM5: RWRegister<u32>,

    /// Stimulus Port Register 6
    pub STIM6: RWRegister<u32>,

    /// Stimulus Port Register 7
    pub STIM7: RWRegister<u32>,

    /// Stimulus Port Register 8
    pub STIM8: RWRegister<u32>,

    /// Stimulus Port Register 9
    pub STIM9: RWRegister<u32>,

    /// Stimulus Port Register 10
    pub STIM10: RWRegister<u32>,

    /// Stimulus Port Register 11
    pub STIM11: RWRegister<u32>,

    /// Stimulus Port Register 12
    pub STIM12: RWRegister<u32>,

    /// Stimulus Port Register 13
    pub STIM13: RWRegister<u32>,

    /// Stimulus Port Register 14
    pub STIM14: RWRegister<u32>,

    /// Stimulus Port Register 15
    pub STIM15: RWRegister<u32>,

    /// Stimulus Port Register 16
    pub STIM16: RWRegister<u32>,

    /// Stimulus Port Register 17
    pub STIM17: RWRegister<u32>,

    /// Stimulus Port Register 18
    pub STIM18: RWRegister<u32>,

    /// Stimulus Port Register 19
    pub STIM19: RWRegister<u32>,

    /// Stimulus Port Register 20
    pub STIM20: RWRegister<u32>,

    /// Stimulus Port Register 21
    pub STIM21: RWRegister<u32>,

    /// Stimulus Port Register 22
    pub STIM22: RWRegister<u32>,

    /// Stimulus Port Register 23
    pub STIM23: RWRegister<u32>,

    /// Stimulus Port Register 24
    pub STIM24: RWRegister<u32>,

    /// Stimulus Port Register 25
    pub STIM25: RWRegister<u32>,

    /// Stimulus Port Register 26
    pub STIM26: RWRegister<u32>,

    /// Stimulus Port Register 27
    pub STIM27: RWRegister<u32>,

    /// Stimulus Port Register 28
    pub STIM28: RWRegister<u32>,

    /// Stimulus Port Register 29
    pub STIM29: RWRegister<u32>,

    /// Stimulus Port Register 30
    pub STIM30: RWRegister<u32>,

    /// Stimulus Port Register 31
    pub STIM31: RWRegister<u32>,

    /// Stimulus Port Register 32
    pub STIM32: RWRegister<u32>,

    /// Stimulus Port Register 33
    pub STIM33: RWRegister<u32>,

    /// Stimulus Port Register 34
    pub STIM34: RWRegister<u32>,

    /// Stimulus Port Register 35
    pub STIM35: RWRegister<u32>,

    /// Stimulus Port Register 36
    pub STIM36: RWRegister<u32>,

    /// Stimulus Port Register 37
    pub STIM37: RWRegister<u32>,

    /// Stimulus Port Register 38
    pub STIM38: RWRegister<u32>,

    /// Stimulus Port Register 39
    pub STIM39: RWRegister<u32>,

    /// Stimulus Port Register 40
    pub STIM40: RWRegister<u32>,

    /// Stimulus Port Register 41
    pub STIM41: RWRegister<u32>,

    /// Stimulus Port Register 42
    pub STIM42: RWRegister<u32>,

    /// Stimulus Port Register 43
    pub STIM43: RWRegister<u32>,

    /// Stimulus Port Register 44
    pub STIM44: RWRegister<u32>,

    /// Stimulus Port Register 45
    pub STIM45: RWRegister<u32>,

    /// Stimulus Port Register 46
    pub STIM46: RWRegister<u32>,

    /// Stimulus Port Register 47
    pub STIM47: RWRegister<u32>,

    /// Stimulus Port Register 48
    pub STIM48: RWRegister<u32>,

    /// Stimulus Port Register 49
    pub STIM49: RWRegister<u32>,

    /// Stimulus Port Register 50
    pub STIM50: RWRegister<u32>,

    /// Stimulus Port Register 51
    pub STIM51: RWRegister<u32>,

    /// Stimulus Port Register 52
    pub STIM52: RWRegister<u32>,

    /// Stimulus Port Register 53
    pub STIM53: RWRegister<u32>,

    /// Stimulus Port Register 54
    pub STIM54: RWRegister<u32>,

    /// Stimulus Port Register 55
    pub STIM55: RWRegister<u32>,

    /// Stimulus Port Register 56
    pub STIM56: RWRegister<u32>,

    /// Stimulus Port Register 57
    pub STIM57: RWRegister<u32>,

    /// Stimulus Port Register 58
    pub STIM58: RWRegister<u32>,

    /// Stimulus Port Register 59
    pub STIM59: RWRegister<u32>,

    /// Stimulus Port Register 60
    pub STIM60: RWRegister<u32>,

    /// Stimulus Port Register 61
    pub STIM61: RWRegister<u32>,

    /// Stimulus Port Register 62
    pub STIM62: RWRegister<u32>,

    /// Stimulus Port Register 63
    pub STIM63: RWRegister<u32>,

    /// Stimulus Port Register 64
    pub STIM64: RWRegister<u32>,

    /// Stimulus Port Register 65
    pub STIM65: RWRegister<u32>,

    /// Stimulus Port Register 66
    pub STIM66: RWRegister<u32>,

    /// Stimulus Port Register 67
    pub STIM67: RWRegister<u32>,

    /// Stimulus Port Register 68
    pub STIM68: RWRegister<u32>,

    /// Stimulus Port Register 69
    pub STIM69: RWRegister<u32>,

    /// Stimulus Port Register 70
    pub STIM70: RWRegister<u32>,

    /// Stimulus Port Register 71
    pub STIM71: RWRegister<u32>,

    /// Stimulus Port Register 72
    pub STIM72: RWRegister<u32>,

    /// Stimulus Port Register 73
    pub STIM73: RWRegister<u32>,

    /// Stimulus Port Register 74
    pub STIM74: RWRegister<u32>,

    /// Stimulus Port Register 75
    pub STIM75: RWRegister<u32>,

    /// Stimulus Port Register 76
    pub STIM76: RWRegister<u32>,

    /// Stimulus Port Register 77
    pub STIM77: RWRegister<u32>,

    /// Stimulus Port Register 78
    pub STIM78: RWRegister<u32>,

    /// Stimulus Port Register 79
    pub STIM79: RWRegister<u32>,

    /// Stimulus Port Register 80
    pub STIM80: RWRegister<u32>,

    /// Stimulus Port Register 81
    pub STIM81: RWRegister<u32>,

    /// Stimulus Port Register 82
    pub STIM82: RWRegister<u32>,

    /// Stimulus Port Register 83
    pub STIM83: RWRegister<u32>,

    /// Stimulus Port Register 84
    pub STIM84: RWRegister<u32>,

    /// Stimulus Port Register 85
    pub STIM85: RWRegister<u32>,

    /// Stimulus Port Register 86
    pub STIM86: RWRegister<u32>,

    /// Stimulus Port Register 87
    pub STIM87: RWRegister<u32>,

    /// Stimulus Port Register 88
    pub STIM88: RWRegister<u32>,

    /// Stimulus Port Register 89
    pub STIM89: RWRegister<u32>,

    /// Stimulus Port Register 90
    pub STIM90: RWRegister<u32>,

    /// Stimulus Port Register 91
    pub STIM91: RWRegister<u32>,

    /// Stimulus Port Register 92
    pub STIM92: RWRegister<u32>,

    /// Stimulus Port Register 93
    pub STIM93: RWRegister<u32>,

    /// Stimulus Port Register 94
    pub STIM94: RWRegister<u32>,

    /// Stimulus Port Register 95
    pub STIM95: RWRegister<u32>,

    /// Stimulus Port Register 96
    pub STIM96: RWRegister<u32>,

    /// Stimulus Port Register 97
    pub STIM97: RWRegister<u32>,

    /// Stimulus Port Register 98
    pub STIM98: RWRegister<u32>,

    /// Stimulus Port Register 99
    pub STIM99: RWRegister<u32>,

    /// Stimulus Port Register 100
    pub STIM100: RWRegister<u32>,

    /// Stimulus Port Register 101
    pub STIM101: RWRegister<u32>,

    /// Stimulus Port Register 102
    pub STIM102: RWRegister<u32>,

    /// Stimulus Port Register 103
    pub STIM103: RWRegister<u32>,

    /// Stimulus Port Register 104
    pub STIM104: RWRegister<u32>,

    /// Stimulus Port Register 105
    pub STIM105: RWRegister<u32>,

    /// Stimulus Port Register 106
    pub STIM106: RWRegister<u32>,

    /// Stimulus Port Register 107
    pub STIM107: RWRegister<u32>,

    /// Stimulus Port Register 108
    pub STIM108: RWRegister<u32>,

    /// Stimulus Port Register 109
    pub STIM109: RWRegister<u32>,

    /// Stimulus Port Register 110
    pub STIM110: RWRegister<u32>,

    /// Stimulus Port Register 111
    pub STIM111: RWRegister<u32>,

    /// Stimulus Port Register 112
    pub STIM112: RWRegister<u32>,

    /// Stimulus Port Register 113
    pub STIM113: RWRegister<u32>,

    /// Stimulus Port Register 114
    pub STIM114: RWRegister<u32>,

    /// Stimulus Port Register 115
    pub STIM115: RWRegister<u32>,

    /// Stimulus Port Register 116
    pub STIM116: RWRegister<u32>,

    /// Stimulus Port Register 117
    pub STIM117: RWRegister<u32>,

    /// Stimulus Port Register 118
    pub STIM118: RWRegister<u32>,

    /// Stimulus Port Register 119
    pub STIM119: RWRegister<u32>,

    /// Stimulus Port Register 120
    pub STIM120: RWRegister<u32>,

    /// Stimulus Port Register 121
    pub STIM121: RWRegister<u32>,

    /// Stimulus Port Register 122
    pub STIM122: RWRegister<u32>,

    /// Stimulus Port Register 123
    pub STIM123: RWRegister<u32>,

    /// Stimulus Port Register 124
    pub STIM124: RWRegister<u32>,

    /// Stimulus Port Register 125
    pub STIM125: RWRegister<u32>,

    /// Stimulus Port Register 126
    pub STIM126: RWRegister<u32>,

    /// Stimulus Port Register 127
    pub STIM127: RWRegister<u32>,

    /// Stimulus Port Register 128
    pub STIM128: RWRegister<u32>,

    /// Stimulus Port Register 129
    pub STIM129: RWRegister<u32>,

    /// Stimulus Port Register 130
    pub STIM130: RWRegister<u32>,

    /// Stimulus Port Register 131
    pub STIM131: RWRegister<u32>,

    /// Stimulus Port Register 132
    pub STIM132: RWRegister<u32>,

    /// Stimulus Port Register 133
    pub STIM133: RWRegister<u32>,

    /// Stimulus Port Register 134
    pub STIM134: RWRegister<u32>,

    /// Stimulus Port Register 135
    pub STIM135: RWRegister<u32>,

    /// Stimulus Port Register 136
    pub STIM136: RWRegister<u32>,

    /// Stimulus Port Register 137
    pub STIM137: RWRegister<u32>,

    /// Stimulus Port Register 138
    pub STIM138: RWRegister<u32>,

    /// Stimulus Port Register 139
    pub STIM139: RWRegister<u32>,

    /// Stimulus Port Register 140
    pub STIM140: RWRegister<u32>,

    /// Stimulus Port Register 141
    pub STIM141: RWRegister<u32>,

    /// Stimulus Port Register 142
    pub STIM142: RWRegister<u32>,

    /// Stimulus Port Register 143
    pub STIM143: RWRegister<u32>,

    /// Stimulus Port Register 144
    pub STIM144: RWRegister<u32>,

    /// Stimulus Port Register 145
    pub STIM145: RWRegister<u32>,

    /// Stimulus Port Register 146
    pub STIM146: RWRegister<u32>,

    /// Stimulus Port Register 147
    pub STIM147: RWRegister<u32>,

    /// Stimulus Port Register 148
    pub STIM148: RWRegister<u32>,

    /// Stimulus Port Register 149
    pub STIM149: RWRegister<u32>,

    /// Stimulus Port Register 150
    pub STIM150: RWRegister<u32>,

    /// Stimulus Port Register 151
    pub STIM151: RWRegister<u32>,

    /// Stimulus Port Register 152
    pub STIM152: RWRegister<u32>,

    /// Stimulus Port Register 153
    pub STIM153: RWRegister<u32>,

    /// Stimulus Port Register 154
    pub STIM154: RWRegister<u32>,

    /// Stimulus Port Register 155
    pub STIM155: RWRegister<u32>,

    /// Stimulus Port Register 156
    pub STIM156: RWRegister<u32>,

    /// Stimulus Port Register 157
    pub STIM157: RWRegister<u32>,

    /// Stimulus Port Register 158
    pub STIM158: RWRegister<u32>,

    /// Stimulus Port Register 159
    pub STIM159: RWRegister<u32>,

    /// Stimulus Port Register 160
    pub STIM160: RWRegister<u32>,

    /// Stimulus Port Register 161
    pub STIM161: RWRegister<u32>,

    /// Stimulus Port Register 162
    pub STIM162: RWRegister<u32>,

    /// Stimulus Port Register 163
    pub STIM163: RWRegister<u32>,

    /// Stimulus Port Register 164
    pub STIM164: RWRegister<u32>,

    /// Stimulus Port Register 165
    pub STIM165: RWRegister<u32>,

    /// Stimulus Port Register 166
    pub STIM166: RWRegister<u32>,

    /// Stimulus Port Register 167
    pub STIM167: RWRegister<u32>,

    /// Stimulus Port Register 168
    pub STIM168: RWRegister<u32>,

    /// Stimulus Port Register 169
    pub STIM169: RWRegister<u32>,

    /// Stimulus Port Register 170
    pub STIM170: RWRegister<u32>,

    /// Stimulus Port Register 171
    pub STIM171: RWRegister<u32>,

    /// Stimulus Port Register 172
    pub STIM172: RWRegister<u32>,

    /// Stimulus Port Register 173
    pub STIM173: RWRegister<u32>,

    /// Stimulus Port Register 174
    pub STIM174: RWRegister<u32>,

    /// Stimulus Port Register 175
    pub STIM175: RWRegister<u32>,

    /// Stimulus Port Register 176
    pub STIM176: RWRegister<u32>,

    /// Stimulus Port Register 177
    pub STIM177: RWRegister<u32>,

    /// Stimulus Port Register 178
    pub STIM178: RWRegister<u32>,

    /// Stimulus Port Register 179
    pub STIM179: RWRegister<u32>,

    /// Stimulus Port Register 180
    pub STIM180: RWRegister<u32>,

    /// Stimulus Port Register 181
    pub STIM181: RWRegister<u32>,

    /// Stimulus Port Register 182
    pub STIM182: RWRegister<u32>,

    /// Stimulus Port Register 183
    pub STIM183: RWRegister<u32>,

    /// Stimulus Port Register 184
    pub STIM184: RWRegister<u32>,

    /// Stimulus Port Register 185
    pub STIM185: RWRegister<u32>,

    /// Stimulus Port Register 186
    pub STIM186: RWRegister<u32>,

    /// Stimulus Port Register 187
    pub STIM187: RWRegister<u32>,

    /// Stimulus Port Register 188
    pub STIM188: RWRegister<u32>,

    /// Stimulus Port Register 189
    pub STIM189: RWRegister<u32>,

    /// Stimulus Port Register 190
    pub STIM190: RWRegister<u32>,

    /// Stimulus Port Register 191
    pub STIM191: RWRegister<u32>,

    /// Stimulus Port Register 192
    pub STIM192: RWRegister<u32>,

    /// Stimulus Port Register 193
    pub STIM193: RWRegister<u32>,

    /// Stimulus Port Register 194
    pub STIM194: RWRegister<u32>,

    /// Stimulus Port Register 195
    pub STIM195: RWRegister<u32>,

    /// Stimulus Port Register 196
    pub STIM196: RWRegister<u32>,

    /// Stimulus Port Register 197
    pub STIM197: RWRegister<u32>,

    /// Stimulus Port Register 198
    pub STIM198: RWRegister<u32>,

    /// Stimulus Port Register 199
    pub STIM199: RWRegister<u32>,

    /// Stimulus Port Register 200
    pub STIM200: RWRegister<u32>,

    /// Stimulus Port Register 201
    pub STIM201: RWRegister<u32>,

    /// Stimulus Port Register 202
    pub STIM202: RWRegister<u32>,

    /// Stimulus Port Register 203
    pub STIM203: RWRegister<u32>,

    /// Stimulus Port Register 204
    pub STIM204: RWRegister<u32>,

    /// Stimulus Port Register 205
    pub STIM205: RWRegister<u32>,

    /// Stimulus Port Register 206
    pub STIM206: RWRegister<u32>,

    /// Stimulus Port Register 207
    pub STIM207: RWRegister<u32>,

    /// Stimulus Port Register 208
    pub STIM208: RWRegister<u32>,

    /// Stimulus Port Register 209
    pub STIM209: RWRegister<u32>,

    /// Stimulus Port Register 210
    pub STIM210: RWRegister<u32>,

    /// Stimulus Port Register 211
    pub STIM211: RWRegister<u32>,

    /// Stimulus Port Register 212
    pub STIM212: RWRegister<u32>,

    /// Stimulus Port Register 213
    pub STIM213: RWRegister<u32>,

    /// Stimulus Port Register 214
    pub STIM214: RWRegister<u32>,

    /// Stimulus Port Register 215
    pub STIM215: RWRegister<u32>,

    /// Stimulus Port Register 216
    pub STIM216: RWRegister<u32>,

    /// Stimulus Port Register 217
    pub STIM217: RWRegister<u32>,

    /// Stimulus Port Register 218
    pub STIM218: RWRegister<u32>,

    /// Stimulus Port Register 219
    pub STIM219: RWRegister<u32>,

    /// Stimulus Port Register 220
    pub STIM220: RWRegister<u32>,

    /// Stimulus Port Register 221
    pub STIM221: RWRegister<u32>,

    /// Stimulus Port Register 222
    pub STIM222: RWRegister<u32>,

    /// Stimulus Port Register 223
    pub STIM223: RWRegister<u32>,

    /// Stimulus Port Register 224
    pub STIM224: RWRegister<u32>,

    /// Stimulus Port Register 225
    pub STIM225: RWRegister<u32>,

    /// Stimulus Port Register 226
    pub STIM226: RWRegister<u32>,

    /// Stimulus Port Register 227
    pub STIM227: RWRegister<u32>,

    /// Stimulus Port Register 228
    pub STIM228: RWRegister<u32>,

    /// Stimulus Port Register 229
    pub STIM229: RWRegister<u32>,

    /// Stimulus Port Register 230
    pub STIM230: RWRegister<u32>,

    /// Stimulus Port Register 231
    pub STIM231: RWRegister<u32>,

    /// Stimulus Port Register 232
    pub STIM232: RWRegister<u32>,

    /// Stimulus Port Register 233
    pub STIM233: RWRegister<u32>,

    /// Stimulus Port Register 234
    pub STIM234: RWRegister<u32>,

    /// Stimulus Port Register 235
    pub STIM235: RWRegister<u32>,

    /// Stimulus Port Register 236
    pub STIM236: RWRegister<u32>,

    /// Stimulus Port Register 237
    pub STIM237: RWRegister<u32>,

    /// Stimulus Port Register 238
    pub STIM238: RWRegister<u32>,

    /// Stimulus Port Register 239
    pub STIM239: RWRegister<u32>,

    /// Stimulus Port Register 240
    pub STIM240: RWRegister<u32>,

    /// Stimulus Port Register 241
    pub STIM241: RWRegister<u32>,

    /// Stimulus Port Register 242
    pub STIM242: RWRegister<u32>,

    /// Stimulus Port Register 243
    pub STIM243: RWRegister<u32>,

    /// Stimulus Port Register 244
    pub STIM244: RWRegister<u32>,

    /// Stimulus Port Register 245
    pub STIM245: RWRegister<u32>,

    /// Stimulus Port Register 246
    pub STIM246: RWRegister<u32>,

    /// Stimulus Port Register 247
    pub STIM247: RWRegister<u32>,

    /// Stimulus Port Register 248
    pub STIM248: RWRegister<u32>,

    /// Stimulus Port Register 249
    pub STIM249: RWRegister<u32>,

    /// Stimulus Port Register 250
    pub STIM250: RWRegister<u32>,

    /// Stimulus Port Register 251
    pub STIM251: RWRegister<u32>,

    /// Stimulus Port Register 252
    pub STIM252: RWRegister<u32>,

    /// Stimulus Port Register 253
    pub STIM253: RWRegister<u32>,

    /// Stimulus Port Register 254
    pub STIM254: RWRegister<u32>,

    /// Stimulus Port Register 255
    pub STIM255: RWRegister<u32>,

    _reserved1: [u32; 640],

    /// Trace Enable Register 0
    pub TER0: RWRegister<u32>,

    /// Trace Enable Register 1
    pub TER1: RWRegister<u32>,

    /// Trace Enable Register 2
    pub TER2: RWRegister<u32>,

    /// Trace Enable Register 3
    pub TER3: RWRegister<u32>,

    /// Trace Enable Register 4
    pub TER4: RWRegister<u32>,

    /// Trace Enable Register 5
    pub TER5: RWRegister<u32>,

    /// Trace Enable Register 6
    pub TER6: RWRegister<u32>,

    /// Trace Enable Register 7
    pub TER7: RWRegister<u32>,

    _reserved2: [u32; 8],

    /// Trace Privilege Register
    pub TPR: RWRegister<u32>,

    _reserved3: [u32; 15],

    /// Trace Control Register
    pub TCR: RWRegister<u32>,

    _reserved4: [u32; 75],

    /// Lock Access Register
    pub LAR: WORegister<u32>,

    /// Lock Status Register
    pub LSR: RORegister<u32>,
}
pub struct ResetValues {
    pub STIM0: u32,
    pub STIM1: u32,
    pub STIM2: u32,
    pub STIM3: u32,
    pub STIM4: u32,
    pub STIM5: u32,
    pub STIM6: u32,
    pub STIM7: u32,
    pub STIM8: u32,
    pub STIM9: u32,
    pub STIM10: u32,
    pub STIM11: u32,
    pub STIM12: u32,
    pub STIM13: u32,
    pub STIM14: u32,
    pub STIM15: u32,
    pub STIM16: u32,
    pub STIM17: u32,
    pub STIM18: u32,
    pub STIM19: u32,
    pub STIM20: u32,
    pub STIM21: u32,
    pub STIM22: u32,
    pub STIM23: u32,
    pub STIM24: u32,
    pub STIM25: u32,
    pub STIM26: u32,
    pub STIM27: u32,
    pub STIM28: u32,
    pub STIM29: u32,
    pub STIM30: u32,
    pub STIM31: u32,
    pub STIM32: u32,
    pub STIM33: u32,
    pub STIM34: u32,
    pub STIM35: u32,
    pub STIM36: u32,
    pub STIM37: u32,
    pub STIM38: u32,
    pub STIM39: u32,
    pub STIM40: u32,
    pub STIM41: u32,
    pub STIM42: u32,
    pub STIM43: u32,
    pub STIM44: u32,
    pub STIM45: u32,
    pub STIM46: u32,
    pub STIM47: u32,
    pub STIM48: u32,
    pub STIM49: u32,
    pub STIM50: u32,
    pub STIM51: u32,
    pub STIM52: u32,
    pub STIM53: u32,
    pub STIM54: u32,
    pub STIM55: u32,
    pub STIM56: u32,
    pub STIM57: u32,
    pub STIM58: u32,
    pub STIM59: u32,
    pub STIM60: u32,
    pub STIM61: u32,
    pub STIM62: u32,
    pub STIM63: u32,
    pub STIM64: u32,
    pub STIM65: u32,
    pub STIM66: u32,
    pub STIM67: u32,
    pub STIM68: u32,
    pub STIM69: u32,
    pub STIM70: u32,
    pub STIM71: u32,
    pub STIM72: u32,
    pub STIM73: u32,
    pub STIM74: u32,
    pub STIM75: u32,
    pub STIM76: u32,
    pub STIM77: u32,
    pub STIM78: u32,
    pub STIM79: u32,
    pub STIM80: u32,
    pub STIM81: u32,
    pub STIM82: u32,
    pub STIM83: u32,
    pub STIM84: u32,
    pub STIM85: u32,
    pub STIM86: u32,
    pub STIM87: u32,
    pub STIM88: u32,
    pub STIM89: u32,
    pub STIM90: u32,
    pub STIM91: u32,
    pub STIM92: u32,
    pub STIM93: u32,
    pub STIM94: u32,
    pub STIM95: u32,
    pub STIM96: u32,
    pub STIM97: u32,
    pub STIM98: u32,
    pub STIM99: u32,
    pub STIM100: u32,
    pub STIM101: u32,
    pub STIM102: u32,
    pub STIM103: u32,
    pub STIM104: u32,
    pub STIM105: u32,
    pub STIM106: u32,
    pub STIM107: u32,
    pub STIM108: u32,
    pub STIM109: u32,
    pub STIM110: u32,
    pub STIM111: u32,
    pub STIM112: u32,
    pub STIM113: u32,
    pub STIM114: u32,
    pub STIM115: u32,
    pub STIM116: u32,
    pub STIM117: u32,
    pub STIM118: u32,
    pub STIM119: u32,
    pub STIM120: u32,
    pub STIM121: u32,
    pub STIM122: u32,
    pub STIM123: u32,
    pub STIM124: u32,
    pub STIM125: u32,
    pub STIM126: u32,
    pub STIM127: u32,
    pub STIM128: u32,
    pub STIM129: u32,
    pub STIM130: u32,
    pub STIM131: u32,
    pub STIM132: u32,
    pub STIM133: u32,
    pub STIM134: u32,
    pub STIM135: u32,
    pub STIM136: u32,
    pub STIM137: u32,
    pub STIM138: u32,
    pub STIM139: u32,
    pub STIM140: u32,
    pub STIM141: u32,
    pub STIM142: u32,
    pub STIM143: u32,
    pub STIM144: u32,
    pub STIM145: u32,
    pub STIM146: u32,
    pub STIM147: u32,
    pub STIM148: u32,
    pub STIM149: u32,
    pub STIM150: u32,
    pub STIM151: u32,
    pub STIM152: u32,
    pub STIM153: u32,
    pub STIM154: u32,
    pub STIM155: u32,
    pub STIM156: u32,
    pub STIM157: u32,
    pub STIM158: u32,
    pub STIM159: u32,
    pub STIM160: u32,
    pub STIM161: u32,
    pub STIM162: u32,
    pub STIM163: u32,
    pub STIM164: u32,
    pub STIM165: u32,
    pub STIM166: u32,
    pub STIM167: u32,
    pub STIM168: u32,
    pub STIM169: u32,
    pub STIM170: u32,
    pub STIM171: u32,
    pub STIM172: u32,
    pub STIM173: u32,
    pub STIM174: u32,
    pub STIM175: u32,
    pub STIM176: u32,
    pub STIM177: u32,
    pub STIM178: u32,
    pub STIM179: u32,
    pub STIM180: u32,
    pub STIM181: u32,
    pub STIM182: u32,
    pub STIM183: u32,
    pub STIM184: u32,
    pub STIM185: u32,
    pub STIM186: u32,
    pub STIM187: u32,
    pub STIM188: u32,
    pub STIM189: u32,
    pub STIM190: u32,
    pub STIM191: u32,
    pub STIM192: u32,
    pub STIM193: u32,
    pub STIM194: u32,
    pub STIM195: u32,
    pub STIM196: u32,
    pub STIM197: u32,
    pub STIM198: u32,
    pub STIM199: u32,
    pub STIM200: u32,
    pub STIM201: u32,
    pub STIM202: u32,
    pub STIM203: u32,
    pub STIM204: u32,
    pub STIM205: u32,
    pub STIM206: u32,
    pub STIM207: u32,
    pub STIM208: u32,
    pub STIM209: u32,
    pub STIM210: u32,
    pub STIM211: u32,
    pub STIM212: u32,
    pub STIM213: u32,
    pub STIM214: u32,
    pub STIM215: u32,
    pub STIM216: u32,
    pub STIM217: u32,
    pub STIM218: u32,
    pub STIM219: u32,
    pub STIM220: u32,
    pub STIM221: u32,
    pub STIM222: u32,
    pub STIM223: u32,
    pub STIM224: u32,
    pub STIM225: u32,
    pub STIM226: u32,
    pub STIM227: u32,
    pub STIM228: u32,
    pub STIM229: u32,
    pub STIM230: u32,
    pub STIM231: u32,
    pub STIM232: u32,
    pub STIM233: u32,
    pub STIM234: u32,
    pub STIM235: u32,
    pub STIM236: u32,
    pub STIM237: u32,
    pub STIM238: u32,
    pub STIM239: u32,
    pub STIM240: u32,
    pub STIM241: u32,
    pub STIM242: u32,
    pub STIM243: u32,
    pub STIM244: u32,
    pub STIM245: u32,
    pub STIM246: u32,
    pub STIM247: u32,
    pub STIM248: u32,
    pub STIM249: u32,
    pub STIM250: u32,
    pub STIM251: u32,
    pub STIM252: u32,
    pub STIM253: u32,
    pub STIM254: u32,
    pub STIM255: u32,
    pub TER0: u32,
    pub TER1: u32,
    pub TER2: u32,
    pub TER3: u32,
    pub TER4: u32,
    pub TER5: u32,
    pub TER6: u32,
    pub TER7: u32,
    pub TPR: u32,
    pub TCR: u32,
    pub LAR: u32,
    pub LSR: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
