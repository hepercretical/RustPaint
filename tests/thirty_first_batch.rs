use mspaint_port::{
    declaration_support::*, declarations31::*, recovered::call_support::MemoryImage,
};
#[test]
fn original_c_layouts_and_bitfield_images() {
    assert_eq!(t_33f191c4e08a::SIZE, 4);
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_33f191c4e08a::SIZE],
        };
        t_33f191c4e08a::F_OFFSETTODIRECTORY
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_33f191c4e08a::F_DATAISDIRECTORY
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        assert_eq!(memory.bytes, vec![0x00, 0x00, 0x00, 0x80]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_33f191c4e08a::SIZE],
        };
        t_33f191c4e08a::F_OFFSETTODIRECTORY
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_33f191c4e08a::F_DATAISDIRECTORY
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        assert_eq!(memory.bytes, vec![0xff, 0xff, 0xff, 0x7f]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_33f191c4e08a::SIZE],
        };
        t_33f191c4e08a::F_OFFSETTODIRECTORY
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_33f191c4e08a::F_DATAISDIRECTORY
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        assert_eq!(memory.bytes, vec![0xf0, 0xde, 0xbc, 0x9a]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_33f191c4e08a::SIZE],
        };
        t_33f191c4e08a::F_OFFSETTODIRECTORY
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_33f191c4e08a::F_DATAISDIRECTORY
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        assert_eq!(memory.bytes, vec![0x55, 0x55, 0x55, 0x55]);
    }
    assert_eq!(t_0a25ff837faa::SIZE, 4);
    assert_eq!(t_0a25ff837faa::F_OFFSETTODATA.offset, 0);
    assert_eq!(
        t_0a25ff837faa::F_IMAGE_RESOURCE_DIRECTORY_ENTRY_DIRECTORYSTRUCT.offset,
        0
    );
    assert_eq!(t_68762d4fce91::SIZE, 16);
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_68762d4fce91::SIZE],
        };
        t_68762d4fce91::F_DEPTH
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_68762d4fce91::F_SEQUENCE
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_68762d4fce91::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_68762d4fce91::F_RESERVED
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_68762d4fce91::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_68762d4fce91::SIZE],
        };
        t_68762d4fce91::F_DEPTH
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_68762d4fce91::F_SEQUENCE
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_68762d4fce91::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_68762d4fce91::F_RESERVED
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_68762d4fce91::F_NEXTENTRY
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0xff, 0xff, 0xf0, 0xde, 0xbc, 0x9a, 0x78, 0x56, 0xf1, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_68762d4fce91::SIZE],
        };
        t_68762d4fce91::F_DEPTH
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_68762d4fce91::F_SEQUENCE
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_68762d4fce91::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_68762d4fce91::F_RESERVED
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_68762d4fce91::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0xf0, 0xde, 0x55, 0x55, 0x55, 0x55, 0xaa, 0xaa, 0x0e, 0xef, 0xcd, 0xab, 0x89, 0x67,
                0x45, 0x23
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_68762d4fce91::SIZE],
        };
        t_68762d4fce91::F_DEPTH
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_68762d4fce91::F_SEQUENCE
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_68762d4fce91::F_HEADERTYPE
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_68762d4fce91::F_RESERVED
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_68762d4fce91::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0x55, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x55, 0x55, 0x55, 0xa5, 0xaa,
                0xaa, 0xaa
            ]
        );
    }
    assert_eq!(t_6650f3d6fe52::SIZE, 16);
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_6650f3d6fe52::SIZE],
        };
        t_6650f3d6fe52::F_DEPTH
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_6650f3d6fe52::F_SEQUENCE
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_6650f3d6fe52::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_6650f3d6fe52::F_INIT
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_6650f3d6fe52::F_RESERVED
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_6650f3d6fe52::F_NEXTENTRY
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf2, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_6650f3d6fe52::SIZE],
        };
        t_6650f3d6fe52::F_DEPTH
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_6650f3d6fe52::F_SEQUENCE
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_6650f3d6fe52::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_6650f3d6fe52::F_INIT
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_6650f3d6fe52::F_RESERVED
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_6650f3d6fe52::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0xff, 0xff, 0xf0, 0xde, 0xbc, 0x9a, 0x78, 0x56, 0x0d, 0xef, 0xcd, 0xab, 0x89, 0x67,
                0x45, 0x23
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_6650f3d6fe52::SIZE],
        };
        t_6650f3d6fe52::F_DEPTH
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_6650f3d6fe52::F_SEQUENCE
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_6650f3d6fe52::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_6650f3d6fe52::F_INIT
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_6650f3d6fe52::F_RESERVED
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_6650f3d6fe52::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0xf0, 0xde, 0x55, 0x55, 0x55, 0x55, 0xaa, 0xaa, 0x52, 0x55, 0x55, 0x55, 0xa5, 0xaa,
                0xaa, 0xaa
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_6650f3d6fe52::SIZE],
        };
        t_6650f3d6fe52::F_DEPTH
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_6650f3d6fe52::F_SEQUENCE
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_6650f3d6fe52::F_HEADERTYPE
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_6650f3d6fe52::F_INIT
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_6650f3d6fe52::F_RESERVED
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_6650f3d6fe52::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0x55, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }
    assert_eq!(t_1d67573570b5::SIZE, 16);
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_1d67573570b5::SIZE],
        };
        t_1d67573570b5::F_DEPTH
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_1d67573570b5::F_SEQUENCE
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_1d67573570b5::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_1d67573570b5::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_1d67573570b5::F_INIT
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_1d67573570b5::F_RESERVED
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_1d67573570b5::F_REGION
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0x00, 0x00, 0xff, 0xe1, 0xbd, 0x79, 0x35, 0xf1, 0xfd, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0x1f
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_1d67573570b5::SIZE],
        };
        t_1d67573570b5::F_DEPTH
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_1d67573570b5::F_SEQUENCE
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_1d67573570b5::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_1d67573570b5::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_1d67573570b5::F_INIT
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_1d67573570b5::F_RESERVED
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_1d67573570b5::F_REGION
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0xff, 0xff, 0xf0, 0xaa, 0xaa, 0xaa, 0xaa, 0x54, 0xc2, 0x7b, 0xf3, 0x6a, 0xe2, 0x59,
                0xd1, 0xa8
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_1d67573570b5::SIZE],
        };
        t_1d67573570b5::F_DEPTH
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_1d67573570b5::F_SEQUENCE
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_1d67573570b5::F_NEXTENTRY
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_1d67573570b5::F_HEADERTYPE
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_1d67573570b5::F_INIT
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_1d67573570b5::F_RESERVED
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_1d67573570b5::F_REGION
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0xf0, 0xde, 0x55, 0x01, 0x00, 0x00, 0x00, 0x00, 0x55, 0x55, 0x55, 0x55, 0xa9, 0xaa,
                0xaa, 0x0a
            ]
        );
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_1d67573570b5::SIZE],
        };
        t_1d67573570b5::F_DEPTH
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_1d67573570b5::F_SEQUENCE
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_1d67573570b5::F_NEXTENTRY
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_1d67573570b5::F_HEADERTYPE
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_1d67573570b5::F_INIT
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_1d67573570b5::F_RESERVED
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_1d67573570b5::F_REGION
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        assert_eq!(
            memory.bytes,
            vec![
                0x55, 0x55, 0x00, 0xfe, 0xff, 0xff, 0xff, 0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0xe0
            ]
        );
    }
    assert_eq!(t_4f57b4157ebe::SIZE, 4);
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4f57b4157ebe::SIZE],
        };
        t_4f57b4157ebe::F_LONGFUNCTION
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_4f57b4157ebe::F_PERSISTENT
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_4f57b4157ebe::F_PRIVATE
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        assert_eq!(memory.bytes, vec![0xc2, 0x7b, 0xf3, 0x6a]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4f57b4157ebe::SIZE],
        };
        t_4f57b4157ebe::F_LONGFUNCTION
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_4f57b4157ebe::F_PERSISTENT
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_4f57b4157ebe::F_PRIVATE
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        assert_eq!(memory.bytes, vec![0x55, 0x55, 0x55, 0x55]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4f57b4157ebe::SIZE],
        };
        t_4f57b4157ebe::F_LONGFUNCTION
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_4f57b4157ebe::F_PERSISTENT
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_4f57b4157ebe::F_PRIVATE
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        assert_eq!(memory.bytes, vec![0x02, 0x00, 0x00, 0x00]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4f57b4157ebe::SIZE],
        };
        t_4f57b4157ebe::F_LONGFUNCTION
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_4f57b4157ebe::F_PERSISTENT
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_4f57b4157ebe::F_PRIVATE
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        assert_eq!(memory.bytes, vec![0xfd, 0xff, 0xff, 0xff]);
    }
    assert_eq!(t_6283f260653f::SIZE, 4);
    assert_eq!(t_6283f260653f::F_FLAGS.offset, 0);
    assert_eq!(t_6283f260653f::F_S.offset, 0);
    assert_eq!(t_15f3b5475a02::SIZE, 72);
    assert_eq!(t_15f3b5475a02::F_VERSION.offset, 0);
    assert_eq!(t_15f3b5475a02::F_POOL.offset, 8);
    assert_eq!(t_15f3b5475a02::F_CLEANUPGROUP.offset, 16);
    assert_eq!(t_15f3b5475a02::F_CLEANUPGROUPCANCELCALLBACK.offset, 24);
    assert_eq!(t_15f3b5475a02::F_RACEDLL.offset, 32);
    assert_eq!(t_15f3b5475a02::F_ACTIVATIONCONTEXT.offset, 40);
    assert_eq!(t_15f3b5475a02::F_FINALIZATIONCALLBACK.offset, 48);
    assert_eq!(t_15f3b5475a02::F_U.offset, 56);
    assert_eq!(t_15f3b5475a02::F_CALLBACKPRIORITY.offset, 60);
    assert_eq!(t_15f3b5475a02::F_SIZE.offset, 64);
    assert_eq!(t_5be79252e409::SIZE, 16);
    assert_eq!(t_5be79252e409::F_S.offset, 0);
    assert_eq!(t_5be79252e409::F_HEADER8.offset, 0);
    assert_eq!(t_5be79252e409::F_HEADER16.offset, 0);
    assert_eq!(t_5be79252e409::F_HEADERX64.offset, 0);
    assert_eq!(t_4467f88a5a08::SIZE, 4);
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4467f88a5a08::SIZE],
        };
        t_4467f88a5a08::F_NAMEOFFSET
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        t_4467f88a5a08::F_NAMEISSTRING
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        assert_eq!(memory.bytes, vec![0x00, 0x00, 0x00, 0x80]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4467f88a5a08::SIZE],
        };
        t_4467f88a5a08::F_NAMEOFFSET
            .store(&mut memory, 0x1000, u64::MAX)
            .unwrap();
        t_4467f88a5a08::F_NAMEISSTRING
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        assert_eq!(memory.bytes, vec![0xff, 0xff, 0xff, 0x7f]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4467f88a5a08::SIZE],
        };
        t_4467f88a5a08::F_NAMEOFFSET
            .store(&mut memory, 0x1000, 0x123456789abcdef0)
            .unwrap();
        t_4467f88a5a08::F_NAMEISSTRING
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        assert_eq!(memory.bytes, vec![0xf0, 0xde, 0xbc, 0x9a]);
    }
    {
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; t_4467f88a5a08::SIZE],
        };
        t_4467f88a5a08::F_NAMEOFFSET
            .store(&mut memory, 0x1000, 0xaaaaaaaa55555555)
            .unwrap();
        t_4467f88a5a08::F_NAMEISSTRING
            .store(&mut memory, 0x1000, 0)
            .unwrap();
        assert_eq!(memory.bytes, vec![0x55, 0x55, 0x55, 0x55]);
    }
    assert_eq!(t_4c02c7411dc1::SIZE, 240);
    assert_eq!(t_4c02c7411dc1::F_MAGIC.offset, 0);
    assert_eq!(t_4c02c7411dc1::F_MAJORLINKERVERSION.offset, 2);
    assert_eq!(t_4c02c7411dc1::F_MINORLINKERVERSION.offset, 3);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFCODE.offset, 4);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFINITIALIZEDDATA.offset, 8);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFUNINITIALIZEDDATA.offset, 12);
    assert_eq!(t_4c02c7411dc1::F_ADDRESSOFENTRYPOINT.offset, 16);
    assert_eq!(t_4c02c7411dc1::F_BASEOFCODE.offset, 20);
    assert_eq!(t_4c02c7411dc1::F_IMAGEBASE.offset, 24);
    assert_eq!(t_4c02c7411dc1::F_SECTIONALIGNMENT.offset, 32);
    assert_eq!(t_4c02c7411dc1::F_FILEALIGNMENT.offset, 36);
    assert_eq!(t_4c02c7411dc1::F_MAJOROPERATINGSYSTEMVERSION.offset, 40);
    assert_eq!(t_4c02c7411dc1::F_MINOROPERATINGSYSTEMVERSION.offset, 42);
    assert_eq!(t_4c02c7411dc1::F_MAJORIMAGEVERSION.offset, 44);
    assert_eq!(t_4c02c7411dc1::F_MINORIMAGEVERSION.offset, 46);
    assert_eq!(t_4c02c7411dc1::F_MAJORSUBSYSTEMVERSION.offset, 48);
    assert_eq!(t_4c02c7411dc1::F_MINORSUBSYSTEMVERSION.offset, 50);
    assert_eq!(t_4c02c7411dc1::F_WIN32VERSIONVALUE.offset, 52);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFIMAGE.offset, 56);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFHEADERS.offset, 60);
    assert_eq!(t_4c02c7411dc1::F_CHECKSUM.offset, 64);
    assert_eq!(t_4c02c7411dc1::F_SUBSYSTEM.offset, 68);
    assert_eq!(t_4c02c7411dc1::F_DLLCHARACTERISTICS.offset, 70);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFSTACKRESERVE.offset, 72);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFSTACKCOMMIT.offset, 80);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFHEAPRESERVE.offset, 88);
    assert_eq!(t_4c02c7411dc1::F_SIZEOFHEAPCOMMIT.offset, 96);
    assert_eq!(t_4c02c7411dc1::F_LOADERFLAGS.offset, 104);
    assert_eq!(t_4c02c7411dc1::F_NUMBEROFRVAANDSIZES.offset, 108);
    assert_eq!(t_4c02c7411dc1::F_DATADIRECTORY.offset, 112);
    assert_eq!(t_807803698433::SIZE, 264);
    assert_eq!(t_807803698433::F_SIGNATURE.offset, 0);
    assert_eq!(t_807803698433::F_FILEHEADER.offset, 4);
    assert_eq!(t_807803698433::F_OPTIONALHEADER.offset, 24);
    assert_eq!(t_286b2bf6be2e::SIZE, 256);
    assert_eq!(t_286b2bf6be2e::F_SIZE.offset, 0);
    assert_eq!(t_286b2bf6be2e::F_TIMEDATESTAMP.offset, 4);
    assert_eq!(t_286b2bf6be2e::F_MAJORVERSION.offset, 8);
    assert_eq!(t_286b2bf6be2e::F_MINORVERSION.offset, 10);
    assert_eq!(t_286b2bf6be2e::F_GLOBALFLAGSCLEAR.offset, 12);
    assert_eq!(t_286b2bf6be2e::F_GLOBALFLAGSSET.offset, 16);
    assert_eq!(t_286b2bf6be2e::F_CRITICALSECTIONDEFAULTTIMEOUT.offset, 20);
    assert_eq!(t_286b2bf6be2e::F_DECOMMITFREEBLOCKTHRESHOLD.offset, 24);
    assert_eq!(t_286b2bf6be2e::F_DECOMMITTOTALFREETHRESHOLD.offset, 32);
    assert_eq!(t_286b2bf6be2e::F_LOCKPREFIXTABLE.offset, 40);
    assert_eq!(t_286b2bf6be2e::F_MAXIMUMALLOCATIONSIZE.offset, 48);
    assert_eq!(t_286b2bf6be2e::F_VIRTUALMEMORYTHRESHOLD.offset, 56);
    assert_eq!(t_286b2bf6be2e::F_PROCESSAFFINITYMASK.offset, 64);
    assert_eq!(t_286b2bf6be2e::F_PROCESSHEAPFLAGS.offset, 72);
    assert_eq!(t_286b2bf6be2e::F_CSDVERSION.offset, 76);
    assert_eq!(t_286b2bf6be2e::F_DEPENDENTLOADFLAGS.offset, 78);
    assert_eq!(t_286b2bf6be2e::F_EDITLIST.offset, 80);
    assert_eq!(t_286b2bf6be2e::F_SECURITYCOOKIE.offset, 88);
    assert_eq!(t_286b2bf6be2e::F_SEHANDLERTABLE.offset, 96);
    assert_eq!(t_286b2bf6be2e::F_SEHANDLERCOUNT.offset, 104);
    assert_eq!(t_286b2bf6be2e::F_GUARDCFCCHECKFUNCTIONPOINTER.offset, 112);
    assert_eq!(t_286b2bf6be2e::F_GUARDCFDISPATCHFUNCTIONPOINTER.offset, 120);
    assert_eq!(t_286b2bf6be2e::F_GUARDCFFUNCTIONTABLE.offset, 128);
    assert_eq!(t_286b2bf6be2e::F_GUARDCFFUNCTIONCOUNT.offset, 136);
    assert_eq!(t_286b2bf6be2e::F_GUARDFLAGS.offset, 144);
    assert_eq!(t_286b2bf6be2e::F_CODEINTEGRITY.offset, 148);
    assert_eq!(t_286b2bf6be2e::F_GUARDADDRESSTAKENIATENTRYTABLE.offset, 160);
    assert_eq!(t_286b2bf6be2e::F_GUARDADDRESSTAKENIATENTRYCOUNT.offset, 168);
    assert_eq!(t_286b2bf6be2e::F_GUARDLONGJUMPTARGETTABLE.offset, 176);
    assert_eq!(t_286b2bf6be2e::F_GUARDLONGJUMPTARGETCOUNT.offset, 184);
    assert_eq!(t_286b2bf6be2e::F_DYNAMICVALUERELOCTABLE.offset, 192);
    assert_eq!(t_286b2bf6be2e::F_CHPEMETADATAPOINTER.offset, 200);
    assert_eq!(t_286b2bf6be2e::F_GUARDRFFAILUREROUTINE.offset, 208);
    assert_eq!(
        t_286b2bf6be2e::F_GUARDRFFAILUREROUTINEFUNCTIONPOINTER.offset,
        216
    );
    assert_eq!(t_286b2bf6be2e::F_DYNAMICVALUERELOCTABLEOFFSET.offset, 224);
    assert_eq!(t_286b2bf6be2e::F_DYNAMICVALUERELOCTABLESECTION.offset, 228);
    assert_eq!(t_286b2bf6be2e::F_RESERVED1.offset, 230);
    assert_eq!(
        t_286b2bf6be2e::F_GUARDRFVERIFYSTACKPOINTERFUNCTIONPOINTER.offset,
        232
    );
    assert_eq!(t_286b2bf6be2e::F_HOTPATCHTABLEOFFSET.offset, 240);
    assert_eq!(t_286b2bf6be2e::F_RESERVED2.offset, 244);
    assert_eq!(t_286b2bf6be2e::F_RESERVED3.offset, 248);
    assert_eq!(t_38ebef70f22f::SIZE, 4);
    assert_eq!(
        t_38ebef70f22f::F_IMAGE_RESOURCE_DIRECTORY_ENTRY_NAMESTRUCT.offset,
        0
    );
    assert_eq!(t_38ebef70f22f::F_NAME.offset, 0);
    assert_eq!(t_38ebef70f22f::F_ID.offset, 0);
    assert_eq!(t_4245c30be2a4::SIZE, 8);
    assert_eq!(t_4245c30be2a4::F_NAMEUNION.offset, 0);
    assert_eq!(t_4245c30be2a4::F_DIRECTORYUNION.offset, 4);
}
// MANUAL TESTS
use mspaint_port::recovered::primitives31::*;
#[test]
fn functions_match_original_c_cases() {
    for line in include_str!("fixtures/primitives31.csv")
        .lines()
        .filter(|l| l.starts_with("L,"))
    {
        let parts: Vec<_> = line.split(',').collect();
        let n: Vec<u64> = parts[1..].iter().map(|s| s.parse().unwrap()).collect();
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0; 512],
        };
        {
            memory
                .write_u64(
                    0x1000,
                    match n[1] {
                        0 => 0,
                        1 => 0x12345678,
                        _ => 0x87654321,
                    },
                )
                .unwrap();
            let mut calls = 0;
            let result = named_l154839(
                0x12345678,
                3,
                &mut memory,
                &mut |s| {
                    assert_eq!(s, "DAT_1401cf478");
                    0x1000
                },
                &mut |s, args, _| {
                    assert_eq!(s, "__scrt_is_ucrt_dll_in_use");
                    assert!(args.is_empty());
                    calls += 1;
                    n[0]
                },
            )
            .unwrap();
            assert_eq!(result, n[2]);
            assert_eq!(memory.read_u64(0x1000).unwrap(), n[3]);
            assert_eq!(calls, 1);
        }
    }
}
#[test]
fn utf16_comparison_matches_original_c_vector_and_scalar_paths() {
    for line in include_str!("fixtures/primitives31.csv")
        .lines()
        .filter(|l| l.starts_with("C,"))
    {
        let p: Vec<i64> = line
            .split(',')
            .skip(1)
            .map(|v| v.parse().unwrap())
            .collect();
        let (feature, count, pos, mode, expected) = (p[0], p[1], p[2], p[3], p[4]);
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0; 512],
        };
        memory.write_u32(0x1000, feature as u32).unwrap();
        for i in 0..96u64 {
            let value = (i * 771 + mode as u64 * 0x7000) as u16;
            memory.write_u16(0x1020 + i * 2, value).unwrap();
            memory.write_u16(0x1100 + i * 2, value).unwrap();
        }
        if pos >= 0 {
            let (a, b) = match mode {
                0 => (0, 0xffff),
                1 => (0x8000, 0x7fff),
                _ => (0xffff, 0),
            };
            memory.write_u16(0x1020 + pos as u64 * 2, a).unwrap();
            memory.write_u16(0x1100 + pos as u64 * 2, b).unwrap();
        }
        assert_eq!(
            fun_14003bc20(0x1020, 0x1100, count as u64, &memory, &mut |s| {
                assert_eq!(s, "DAT_1401d03a4");
                0x1000
            })
            .unwrap(),
            expected as u32,
            "{line}"
        );
    }
}
#[test]
fn comparisons_keep_full_block_reads_and_startup_contention_is_bounded() {
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0; 32],
    };
    memory.write_u32(0x1000, 1).unwrap();
    memory.write_u16(0x1008, 1).unwrap();
    assert!(fun_14003bc20(0x1008, 0x1010, 16, &memory, &mut |_| 0x1000).is_err());
    assert!(fun_14003bc20(0x1008, 0x1018, 8, &memory, &mut |_| 0x1000).is_err());
    memory.write_u64(0x1000, 99).unwrap();
    assert_eq!(
        named_l154839(42, 2, &mut memory, &mut |_| 0x1000, &mut |_, _, _| 1),
        Err(PrimitiveError::SpinLimit)
    );
    assert_eq!(memory.read_u64(0x1000).unwrap(), 99);
}
#[test]
fn bit_fields_preserve_neighbors_and_fail_without_partial_writes() {
    let field = BitField::new(0, 8, 16, 48);
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0xa5; 8],
    };
    field.store(&mut memory, 0x1000, u64::MAX).unwrap();
    assert_eq!(field.load(&memory, 0x1000).unwrap(), 0xffffffffffff);
    assert_eq!(memory.read_u16(0x1000).unwrap(), 0xa5a5);
    let previous = memory.bytes.clone();
    assert!(field.store(&mut memory, 0x1001, 0).is_err());
    assert!(BitField::new(8, 4, 0, 31).load(&memory, u64::MAX).is_err());
    assert_eq!(memory.bytes, previous);
    let tables: [Global<[Address<Unknown>; 2]>; 1] = [G_L3870];
    let table = tables[0].address(&mut |_| 0x1000);
    assert_eq!(table.value, 0x1000);
}
