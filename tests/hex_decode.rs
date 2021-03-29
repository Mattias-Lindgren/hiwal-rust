#[cfg(test)]
mod tests{
    use hiwal_rust::hex_to_color;
    use palette::rgb::Rgb;

    #[test]
    fn hex_to_color_success() {
        let expected = Rgb::new(252f32/255f32, 186f32/255f32, 3f32/255f32);
        let test = hex_to_color("#fcba03".into())
            .expect("fail");
        assert_eq!(expected, test);
    }

    #[test]
    fn hex_to_color_wrong_input() {
        let test = hex_to_color("-#fcba03".into());
        assert!(test.is_none());
    }
}
