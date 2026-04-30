#[allow(dead_code)]
pub fn md5_hex(input: &str) -> String {
    md5(input.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub fn md5(input: &[u8]) -> [u8; 16] {
    md5_via_vec(&mut input.to_vec())
}
pub fn md5_via_vec(input: &mut Vec<u8>) -> [u8; 16] {
    // // : All variables are unsigned 32 bit and wrap modulo 2^32 when calculating
    // var int s[64], K[64]
    // var int i

    // // s specifies the per-round shift amounts
    // s[ 0..15] := { 7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22 }
    // s[16..31] := { 5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20 }
    // s[32..47] := { 4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23 }
    // s[48..63] := { 6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21 }

    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    // // Use binary integer part of the sines of integers (Radians) as constants:
    // for i from 0 to 63 do
    //     K[i] := floor(2^32 × abs(sin(i + 1)))
    // end for
    // // (Or just use the following precomputed table):
    // K[ 0.. 3] := { 0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee }
    // K[ 4.. 7] := { 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501 }
    // K[ 8..11] := { 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be }
    // K[12..15] := { 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821 }
    // K[16..19] := { 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa }
    // K[20..23] := { 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8 }
    // K[24..27] := { 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed }
    // K[28..31] := { 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a }
    // K[32..35] := { 0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c }
    // K[36..39] := { 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70 }
    // K[40..43] := { 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05 }
    // K[44..47] := { 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665 }
    // K[48..51] := { 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039 }
    // K[52..55] := { 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1 }
    // K[56..59] := { 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1 }
    // K[60..63] := { 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 }

    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    // // Initialize variables:
    // var int a0 := 0x67452301   // A
    // var int b0 := 0xefcdab89   // B
    // var int c0 := 0x98badcfe   // C
    // var int d0 := 0x10325476   // D

    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    // // Pre-processing: adding a single 1 bit
    // append "1" bit to message<
    //  // Notice: the input bytes are considered as bit strings,
    //  //  where the first bit is the most significant bit of the byte.[52]

    // // Pre-processing: padding with zeros
    // append "0" bit until message length in bits ≡ 448 (mod 512)

    // // Notice: the two padding steps above are implemented in a simpler way
    //   //  in implementations that only work with complete bytes: append 0x80
    //   //  and pad with 0x00 bytes so that the message length in bytes ≡ 56 (mod 64).

    // assume the length is less than 2^64, otherwise how would we store it in a vector?
    let original_bit_length = (input.len() as u64) * 8;

    // CHANGED to use passed in vector
    // let mut input: Vec<u8> = input.to_vec(); // copy input to a new Vec

    input.push(0x80);

    //  512 bits = 64 bytes, 64 bits = 8 bytes
    let new_size = (((input.len() + 8) / 64) + 1) * 64 - 8;
    input.resize(new_size, 0);

    // while input.len() % 64 != 56 {
    //     input.push(0);
    // }

    // append original length in bits mod 2^64 to message

    input.extend_from_slice(&original_bit_length.to_le_bytes());

    // // Process the message in successive 512-bit chunks:
    // for each 512-bit chunk of padded message do
    for chunk in input.chunks_exact(64) {
        //     break chunk into sixteen 32-bit words M[j], 0 ≤ j ≤ 15
        let m = |g: u16| -> u32 {
            let g = g as usize;
            u32::from_le_bytes(chunk[g * 4..g * 4 + 4].as_array::<4>().unwrap().to_owned())
        };
        //     // Initialize hash value for this chunk:
        //     var int A := a0
        //     var int B := b0
        //     var int C := c0
        //     var int D := d0
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        //     // Main loop:
        //     for i from 0 to 63 do
        for i in 0..64 {
            //         var int F, g
            let mut f: u32;
            let g: u16;
            //         if 0 ≤ i ≤ 15 then
            //             F := (B and C) or ((not B) and D)
            //             g := i
            if (0..16).contains(&i) {
                f = (b & c) | (!b & d);
                g = i;
                //         else if 16 ≤ i ≤ 31 then
                //             F := (D and B) or ((not D) and C)
                //             g := (5×i + 1) mod 16
            } else if (16..32).contains(&i) {
                f = (b & d) | (!d & c);
                g = (5 * i + 1) % 16;
            //         else if 32 ≤ i ≤ 47 then
            //             F := B xor C xor D
            //             g := (3×i + 5) mod 16
            } else if (32..48).contains(&i) {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            //         else if 48 ≤ i ≤ 63 then
            //             F := C xor (B or (not D))
            //             g := (7×i) mod 16
            } else if (48..64).contains(&i) {
                f = c ^ (b | !d);
                g = (7 * i) % 16;
            } else {
                unreachable!();
            };
            //         // Be wary of the below definitions of a,b,c,d
            //         F := F + A + K[i] + M[g]  // M[g] must be a 32-bit block
            //         A := D
            //         D := C
            //         C := B
            //         B := B + leftrotate(F, s[i])
            f = f
                .wrapping_add(a)
                .wrapping_add(K[i as usize])
                .wrapping_add(m(g));
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i as usize]));
            //     end for
        }
        //     // Add this chunk's hash to result so far:
        //     a0 := a0 + A
        //     b0 := b0 + B
        //     c0 := c0 + C
        //     d0 := d0 + D
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
        // end for
    }
    // var char digest[16] := a0 append b0 append c0 append d0 // (Output is in little-endian)
    let mut result: [u8; 16] = [0; 16];
    result[0..4].copy_from_slice(&a0.to_le_bytes());
    result[4..8].copy_from_slice(&b0.to_le_bytes());
    result[8..12].copy_from_slice(&c0.to_le_bytes());
    result[12..16].copy_from_slice(&d0.to_le_bytes());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vectors() {
        assert_eq!(md5_hex(""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(md5_hex("a"), "0cc175b9c0f1b6a831c399e269772661");
        assert_eq!(md5_hex("abc"), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(
            md5_hex("message digest"),
            "f96b697d7cb7938d525a2f31aaf161d0"
        );
        assert_eq!(
            md5_hex("The quick brown fox jumps over the lazy dog"),
            "9e107d9d372bb6826bd81d3542a419d6"
        );
        assert_eq!(
            md5_hex("The quick brown fox jumps over the lazy dog."),
            "e4d909c290d0fb1ca068ffaddf22cbd0"
        );
        // MD5("The quick brown fox jumps over the lazy dog.") =
        // e4d909c290d0fb1ca068ffaddf22cbd0
        assert_eq!(md5_hex("abcdef1"), "5f8b62a2dced0cd28946a9c891ff3e5e");
    }
}
