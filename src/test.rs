extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use super::*;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[cfg(feature = "std")]
macro_rules! test_log {
    ($($arg:tt)*) => {{
        std::println!("{}", alloc::format!($($arg)*));
    }};
}

#[cfg(not(feature = "std"))]
macro_rules! test_log {
    ($($arg:tt)*) => { { let _ = (&$($arg)*); } };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_std_compilation() {
        // Test that basic types work in no_std environment
        let mut map = BTreeMap::new();
        map.insert(1u64, "test");
        assert_eq!(map.get(&1), Some(&"test"));
        
        let mut vec = Vec::new();
        vec.push(42u64);
        assert_eq!(vec.len(), 1);
        
        test_log!("no_std compilation test passed");
    }

    #[test]
    fn test_alloc_format() {
        let formatted = alloc::format!("test_{}", 123);
        assert_eq!(formatted, "test_123");
        test_log!("alloc::format test: {}", formatted);
    }

    #[test]
    fn test_nft_kind_conversion() {
        let kind = NftKind::Common;
        let key = kind.to_key();
        assert_eq!(key, "Common");
        
        let restored = NftKind::from_key(&key).unwrap();
        assert_eq!(restored, kind);
        
        test_log!("NftKind conversion test passed");
    }
}
