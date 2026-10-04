use u8_base_converter::*;

#[test]
fn test_base(){
    assert_eq!("123",Base::new(b"123").as_str());
    assert_eq!("0123456789",DECIMAL.as_printable_ascii());
    assert_eq!(Base::from_base(HEXADECIMAL,0,8).as_str(),OCTAL.as_str());
    assert_eq!(BINARY.as_str(),Base::from_radix(2).as_str());
    assert_eq!(Base::from_str("345").as_printable_ascii(),"345");
    assert_eq!(BASE64.len(),64);
}

#[test]
fn test_numeral(){
    assert_eq!(Numeral::new(b"12",DECIMAL).value_as_str(),Numeral::new_dec(b"12").value_as_str());
    assert_eq!(Numeral::new_hex(b"").base().as_str(),HEXADECIMAL.as_str());
    assert_eq!(Numeral::from_str("abc","abcd123").value_as_printable_ascii(),Numeral::new(b"abc",Base::new(b"abcd123")).value_as_printable_ascii());
    assert_eq!(Numeral::new_dec_from_u128(45).value_as_str(),Numeral::new_dec(b"45").value_as_str());
    assert_eq!(Numeral::new_bin(b"101").converted_to(DECIMAL).value_as_str(),"5");
    assert_eq!(Numeral::new_hex(b"3A").as_u128(),Some(58u128));
    assert_eq!(Numeral::new(b"c5",ALPHANUMERIC).converted_to(UNARY).value_as_str(),"")// Overflow test, since `c5` is greater than 1024.
}