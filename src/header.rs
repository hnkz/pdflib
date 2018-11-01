
// %PDF-
static HEADER_MN: &str = "%PDF-";

#[derive(Debug)]
pub enum Header {
    PDF16,
    PDF17,
}

impl Header {
    pub fn check_header(buf: &Vec<u8>) -> Result<Header, String> {
        use std::str;
        let header = match str::from_utf8(&buf[0..5]) {
            Ok(header) => header,
            Err(err) => {
                return Err(format!("header str::from_utf8 error: {:?}", err));
            }
        };
    
        if header == HEADER_MN {
            let version = match str::from_utf8(&buf[5..8]) {
                Ok(version) => version,
                Err(err) => {
                    return Err(format!("header str::from_utf8 error: {:?}", err));
                }
            };
            match version {
                "1.6" => Ok(Header::PDF16),
                "1.7" => Ok(Header::PDF17),
                _ => Err(String::from("This PDF file version is not supported."))
            }
        } else {
             return Err(String::from("This file is not PDF file."));
        }
    }
}
