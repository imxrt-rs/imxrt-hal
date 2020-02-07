# Many ideas here are taken from https://github.com/stm32-rs/stm32-rs

all: crates

CRATES ?= imxrt1011 imxrt1015 imxrt1021 imxrt1051 imxrt1052 imxrt1061 imxrt1062 imxrt1064
CLEAN_CRATES := $(foreach crate, $(CRATES), clean_$(crate))

svd/.xml:


define crate_template

$(1)/src/lib.rs: svd/$(1).xml
	cd $(1) && svd2rust -i ../svd/$(1).xml
	cd $(1) && rm -rf src
	cd $(1) && form -i lib.rs -o src/ && rm lib.rs
	cd $(1) && cargo fmt

clean_$(1):
	rm -rf $(1)/src/*

$(1): $(1)/src/lib.rs

endef

$(foreach crate,$(CRATES),$(eval $(call crate_template,$(crate))))


crates: $(CRATES)

clean: $(CLEAN_CRATES)
