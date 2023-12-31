use super::method::{Method, MethodError,};
use std::convert::TryFrom;
use std::error::Error;
use std::str;
use std::str::Utf8Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter};
use super::QueryString;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, 
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// impl Request<'buf> {
//     fn from_byte_array(buf: &'buf [u8]) -> Result<Request<'buf>, String> {
//         unimplemented!()
//     }
// }

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>,  Self::Error> {


        let request = str::from_utf8(buf)?;
        match get_next_word(request) {
            Some((method, request)) => {},
            None => return Err(ParseError::InvalidRequest)
        }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        } 

        Ok(Self{
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c =='\r' {
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError{
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError{
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}

