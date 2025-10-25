macro_rules! kernel_shim {
    ($struct:ident, $member:ident, $ret:ty) => {
        kernel_shim! (pub, $member, $struct, $member, $ret);
    };

    ($pub:vis, $struct:ident, $member:ident, $ret:ty) => {
        kernel_shim! ($pub, $member, $struct, $member, $ret);
    };

    ($pub:vis, $fn_name:ident, $struct: ident, $member:ident, $ret:ty) => {
        #[inline(always)]
        $pub fn $fn_name(&self) -> Option<$ret> {
            unsafe {
                if !self.is_null() && paste::paste! {[<shim_ $struct _ $member _exists>]}(self.as_ptr_mut()) {
                    return Some(paste::paste! {[<shim_ $struct _ $member>]}(self.as_ptr_mut()).into());
                }
                None
            }
        }
    };
}

pub(crate) use kernel_shim;

// macro_rules! user_shim {
// 	($pub:vis, $struct:ident, $member:ident, $ret:ty) => {
// 		user_shim!($pub, $member, $struct, $member, $ret);
// 	};

// 	($pub:vis, $fn_name:ident, $struct: ident, $member:ident, $ret:ty) => {
// 		paste::item! {
// 		#[inline(always)]
// 		$pub unsafe fn [<$fn_name _user>] (&self) -> Option<$ret> {
// 			if !self.is_null()
// 				&& [<shim_ $struct _ $member _exists>](self.as_ptr_mut())
// 			{
// 				return Some(paste::paste! {[<shim_ $struct _ $member _user>]}(self.as_ptr_mut()).into());
// 			}
// 			None
// 		}
// 		}
// 	};
// }

// pub(crate) use user_shim;
