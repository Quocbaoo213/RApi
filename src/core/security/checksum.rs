pub const MAGIC_CHECKSUM_XOR: u32 = 1827134112;
pub const CKSUM_MAGIC: u32 = MAGIC_CHECKSUM_XOR;

pub fn compute_checksum(
    cmd: i32,
    sub: i32,
    seq: i32,
    uid: i64,
    target: i64,
    target_type: i32,
    cmsg: i32,
    bb: i32,
    ty: i32,
    ver: i32,
) -> u32 {
    let total = ((bb as u32)
        .wrapping_add(ty as u32)
        .wrapping_add(seq as u32)
        .wrapping_add(uid as u32)
        .wrapping_add(ver as u32)
        .wrapping_add(cmd as u32)
        .wrapping_add(sub as u32)
        .wrapping_add(target as u32)
        .wrapping_add(target_type as u32)
        .wrapping_add(cmsg as u32))
        & 0xFFFFFFFF;
    total ^ MAGIC_CHECKSUM_XOR
}

pub fn compute_inner_checksum(
    cmd: i32,
    sub: i32,
    seq: i32,
    uid: i64,
    bb: i32,
    ty: i32,
    ver: i32,
) -> u32 {
    let total = ((bb as u32)
        .wrapping_add(ty as u32)
        .wrapping_add(seq as u32)
        .wrapping_add(uid as u32)
        .wrapping_add(ver as u32)
        .wrapping_add(cmd as u32)
        .wrapping_add(sub as u32))
        & 0xFFFFFFFF;
    total ^ MAGIC_CHECKSUM_XOR
}
