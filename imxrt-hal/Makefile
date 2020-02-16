DEVICES ?= imxrt1061 imxrt1062 imxrt1064 

# Check vairous feature set combinations here for each SoC in the family
define check_template
.check/$(1).ok:
	@mkdir -p .check
	cargo check --features=$(1) && touch $$@

endef

$(foreach device,$(DEVICES),$(eval $(call check_template,$(device))))

check: $(patsubst %, .check/%.ok, $(DEVICES))

clean-check:
	rm -rf .check

