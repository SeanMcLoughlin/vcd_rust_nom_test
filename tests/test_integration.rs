extern crate vcd_rust;

#[cfg(test)]
mod tests {
    use super::*;
    use vcd_rust::vcd::VCDBuilder;

    #[test]
    fn basic() {
        let file_contents = "$timescale 1 ps $end";
        let exp_vcd = VCDBuilder::default().timescale("1 ps").build().unwrap();
        let act_vcd = vcd_rust::load_from_contents(file_contents).unwrap();
        assert_eq!(exp_vcd, act_vcd);
    }
}
