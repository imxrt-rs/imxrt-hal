// No alt7...

pad!(
    GPIO_B1_00,
    SW_MUX_CTL_PAD_GPIO_B1_00,
    SW_PAD_CTL_PAD_GPIO_B1_00,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_01,
    SW_MUX_CTL_PAD_GPIO_B1_01,
    SW_PAD_CTL_PAD_GPIO_B1_01,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_02,
    SW_MUX_CTL_PAD_GPIO_B1_02,
    SW_PAD_CTL_PAD_GPIO_B1_02,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_03,
    SW_MUX_CTL_PAD_GPIO_B1_03,
    SW_PAD_CTL_PAD_GPIO_B1_03,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

// ... and now we drop alt6...

pad!(
    GPIO_B1_04,
    SW_MUX_CTL_PAD_GPIO_B1_04,
    SW_PAD_CTL_PAD_GPIO_B1_04,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

pad!(
    GPIO_B1_05,
    SW_MUX_CTL_PAD_GPIO_B1_05,
    SW_PAD_CTL_PAD_GPIO_B1_05,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

pad!(
    GPIO_B1_06,
    SW_MUX_CTL_PAD_GPIO_B1_06,
    SW_PAD_CTL_PAD_GPIO_B1_06,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

pad!(
    GPIO_B1_07,
    SW_MUX_CTL_PAD_GPIO_B1_07,
    SW_PAD_CTL_PAD_GPIO_B1_07,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt8, alt9]
);

// ... and alt6 is back!

pad!(
    GPIO_B1_08,
    SW_MUX_CTL_PAD_GPIO_B1_08,
    SW_PAD_CTL_PAD_GPIO_B1_08,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_09,
    SW_MUX_CTL_PAD_GPIO_B1_09,
    SW_PAD_CTL_PAD_GPIO_B1_09,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

// ... buuuuuuuuut now we drop alt8...

pad!(
    GPIO_B1_10,
    SW_MUX_CTL_PAD_GPIO_B1_10,
    SW_PAD_CTL_PAD_GPIO_B1_10,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt9]
);

pad!(
    GPIO_B1_11,
    SW_MUX_CTL_PAD_GPIO_B1_11,
    SW_PAD_CTL_PAD_GPIO_B1_11,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt9]
);

// SIKE THERE'S NO ALT0 HERE!!!!!!!
//
// The compiler caught this, which is aweseome!
// We don't have to worry about adding incorrect
// alternatives; we only need concern ourselves about
// missing alternatives. Thanks, svd2rust!
pad!(
    GPIO_B1_12,
    SW_MUX_CTL_PAD_GPIO_B1_12,
    SW_PAD_CTL_PAD_GPIO_B1_12,
    [alt1, alt2, alt3, alt4, alt5, alt6, alt9]
);

// ... bring back alt0, and alt8.

pad!(
    GPIO_B1_13,
    SW_MUX_CTL_PAD_GPIO_B1_13,
    SW_PAD_CTL_PAD_GPIO_B1_13,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_14,
    SW_MUX_CTL_PAD_GPIO_B1_14,
    SW_PAD_CTL_PAD_GPIO_B1_14,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);

pad!(
    GPIO_B1_15,
    SW_MUX_CTL_PAD_GPIO_B1_15,
    SW_PAD_CTL_PAD_GPIO_B1_15,
    [alt0, alt1, alt2, alt3, alt4, alt5, alt6, alt8, alt9]
);
