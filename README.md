# base64

**Base64** is a group of similar [binary-to-text encoding](https://en.wikipedia.org/wiki/Binary-to-text_encoding) schemes that represent binary data in an ASCII string format by translating it into a radix-64 representation. The term *Base64* originates from a specific [MIME content transfer encoding](https://en.wikipedia.org/wiki/MIME#Content-Transfer-Encoding).

Base64 encoding schemes are commonly used when there is a need to encode binary data that needs to be stored and transferred over media that are designed to deal with ASCII. This is to ensure that the data remain intact without modification during transport.

Each Base64 digit represents exactly 6 bits of data. So, three 8-bits bytes of the input string/binary file (3*8 bits = 24 bits) can be represented by four 6-bit Base64 digits (4×6 = 24 bits).

## API
```rs
/// Encodes ASCCI string into base64 format string
encode<T>(input: T) -> Result<String, Base64Error> where T: AsRef<str>
```

```rs
/// Decodes base64 format string into ASCCI string
decode<T>(input: T) -> Result<String, Base64Error> where T: AsRef<str>
```