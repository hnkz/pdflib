
pub mod pdfreader;
pub mod header;
pub mod object;


use self::header::Header;
use self::object::obj::Obj;

#[derive(Debug)]
pub struct PDF {
    header: Header,
    obj: Vec<Obj>,
}

impl PDF {
    pub fn new(buf: Vec<u8>) -> Result<PDF, String> {
        let mut buf = PDF::get_line(buf);
        let header = match Header::check_header(&buf[0]) {
            Ok(header) => {
                buf.remove(0);   
                header
            }
            Err(err) => return Err(err)
        };

        let mut obj: Vec<Obj> = Vec::new();
        for line in buf {
            
        }


        Ok(PDF {
            header: header,
            obj: obj
        })
    }

    fn get_line(buf: Vec<u8>) -> Vec<Vec<u8>> {
        let mut res: Vec<Vec<u8>> = Vec::new();
        let mut line: Vec<u8> = Vec::new();
        for b in buf {
            // '0x0a'
            if b == 10 {
                // println!("{:x?}", line);
                res.push(line.clone());
                line.clear();
            } else {
                line.push(b);
            }
        }

        res
    }
}