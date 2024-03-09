use luaproc::*;

#[test]
fn point1() {
    #[derive(Debug, LuaProc)]
    #[cfg(target_os = "linux")]
    #[luaproc("tests/point1.lua")]
    struct Point1 {
        x: u16,
        y: u32,
    }

    // tests/struct_1.lua defines a specific Default impl
    let pt1 = Point1::default();
    assert_eq!(pt1.x, u16::MAX);
    assert_eq!(pt1.y, u32::MAX);
}

#[test]
fn point2() {
    #[derive(Debug, Default, LuaProc)]
    #[luaproc("tests/point2.lua")]
    struct Point2<T>
    where
        T: Copy,
        T: std::ops::Add<Output = T>,
        T: Default,
    {
        x0: T,
        x1: T,
        x2: T,
        x3: T,
        x4: T,
        x5: T,
        x6: T,
        x7: T,
        x8: T,
        x9: T,
    }

    // tests/struct_2.lua create a function with adds all fields
    let mut pt2 = Point2::<u8>::default();
    pt2.x0 = 2;
    pt2.x9 = 1;

    assert_eq!(add(&pt2), 3u8);
}

#[test]
fn choice1() {
    #[derive(Debug, LuaProc)]
    #[luaproc("tests/choice1.lua")]
    #[repr(u8)]
    enum Choice {
        Ok = 0,
        Nok = 1 + 2,
    }
}

#[test]
fn enum_2() {
    #[derive(Debug, LuaProc)]
    #[luaproc("tests/enum_2.lua")]
    #[repr(u8)]
    enum Choice {
        #[luaproc(foo)]
        Ok = 0,
        Nok = 1 + 2,
    }
}

#[test]
fn opcode() {
    use std::str::FromStr;

    #[derive(Debug, PartialEq, LuaProc)]
    #[luaproc("tests/opcode.lua")]
    #[repr(u8)]
    enum OpCode {
        Query = 0,  //[RFC1035]
        IQuery = 1, // (Inverse Query, OBSOLETE)	[RFC3425]
        Status = 2, // [RFC1035]
        Unassigned = 3,
        Notify = 4, // [RFC1996]
        Update = 5, // [RFC2136]
        DOS = 6,    // DNS Stateful Operations (DSO)	[RFC8490]
    }

    let op = OpCode::from_str("Update").unwrap();
    assert_eq!(op, OpCode::Update);
    let op = OpCode::from_str("Foo");
    assert!(op.is_err());

    let op = OpCode::try_from(4).unwrap();
    assert_eq!(op, OpCode::Notify);
    let op = OpCode::try_from(10);
    assert!(op.is_err());
}
