(function() {var implementors = {
"allocator_api2":[["impl&lt;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, A: <a class=\"trait\" href=\"allocator_api2/alloc/trait.Allocator.html\" title=\"trait allocator_api2::alloc::Allocator\">Allocator</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"allocator_api2/boxed/struct.Box.html\" title=\"struct allocator_api2::boxed::Box\">Box</a>&lt;T, A&gt;"]],
"bitvec":[["impl&lt;M, T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/ptr/struct.BitPtr.html\" title=\"struct bitvec::ptr::BitPtr\">BitPtr</a>&lt;M, T, O&gt;<span class=\"where fmt-newline\">where\n    M: <a class=\"trait\" href=\"bitvec/ptr/trait.Mutability.html\" title=\"trait bitvec::ptr::Mutability\">Mutability</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,</span>"],["impl&lt;T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/boxed/struct.BitBox.html\" title=\"struct bitvec::boxed::BitBox\">BitBox</a>&lt;T, O&gt;<span class=\"where fmt-newline\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</span>"],["impl&lt;M, T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/ptr/struct.BitRef.html\" title=\"struct bitvec::ptr::BitRef\">BitRef</a>&lt;'_, M, T, O&gt;<span class=\"where fmt-newline\">where\n    M: <a class=\"trait\" href=\"bitvec/ptr/trait.Mutability.html\" title=\"trait bitvec::ptr::Mutability\">Mutability</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,</span>"],["impl&lt;T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;T, O&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,</span>"],["impl&lt;T, O&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"bitvec/vec/struct.BitVec.html\" title=\"struct bitvec::vec::BitVec\">BitVec</a>&lt;T, O&gt;<span class=\"where fmt-newline\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</span>"]],
"boa_gc":[["impl&lt;T: <a class=\"trait\" href=\"boa_gc/trait.Trace.html\" title=\"trait boa_gc::Trace\">Trace</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"boa_gc/struct.Gc.html\" title=\"struct boa_gc::Gc\">Gc</a>&lt;T&gt;"]],
"console":[["impl&lt;D: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"console/struct.StyledObject.html\" title=\"struct console::StyledObject\">StyledObject</a>&lt;D&gt;"]],
"crossbeam_epoch":[["impl&lt;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"crossbeam_epoch/trait.Pointable.html\" title=\"trait crossbeam_epoch::Pointable\">Pointable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"crossbeam_epoch/struct.Shared.html\" title=\"struct crossbeam_epoch::Shared\">Shared</a>&lt;'_, T&gt;"],["impl&lt;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"crossbeam_epoch/trait.Pointable.html\" title=\"trait crossbeam_epoch::Pointable\">Pointable</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"crossbeam_epoch/struct.Atomic.html\" title=\"struct crossbeam_epoch::Atomic\">Atomic</a>&lt;T&gt;"]],
"itertools":[["impl&lt;'a, I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"itertools/structs/struct.Format.html\" title=\"struct itertools::structs::Format\">Format</a>&lt;'a, I&gt;<span class=\"where fmt-newline\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.73.0/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>,</span>"]],
"nalgebra":[["impl&lt;T, R: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, C: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.Dim.html\" title=\"trait nalgebra::base::dimension::Dim\">Dim</a>, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"nalgebra/base/struct.Matrix.html\" title=\"struct nalgebra::base::Matrix\">Matrix</a>&lt;T, R, C, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>,\n    S: <a class=\"trait\" href=\"nalgebra/base/storage/trait.RawStorage.html\" title=\"trait nalgebra::base::storage::RawStorage\">RawStorage</a>&lt;T, R, C&gt;,</span>"]],
"portable_atomic":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"portable_atomic/struct.AtomicPtr.html\" title=\"struct portable_atomic::AtomicPtr\">AtomicPtr</a>&lt;T&gt;"]],
"tinyvec":[["impl&lt;'s, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"tinyvec/struct.SliceVec.html\" title=\"struct tinyvec::SliceVec\">SliceVec</a>&lt;'s, T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>,</span>"],["impl&lt;A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"tinyvec/struct.ArrayVec.html\" title=\"struct tinyvec::ArrayVec\">ArrayVec</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>,</span>"],["impl&lt;A: <a class=\"trait\" href=\"tinyvec/trait.Array.html\" title=\"trait tinyvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"enum\" href=\"tinyvec/enum.TinyVec.html\" title=\"enum tinyvec::TinyVec\">TinyVec</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    A::<a class=\"associatedtype\" href=\"tinyvec/trait.Array.html#associatedtype.Item\" title=\"type tinyvec::Array::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>,</span>"]],
"wasmtime":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wasmtime/struct.ExternRef.html\" title=\"struct wasmtime::ExternRef\">ExternRef</a>"]],
"wasmtime_environ":[],
"wasmtime_runtime":[["impl&lt;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wasmtime_runtime/struct.SendSyncPtr.html\" title=\"struct wasmtime_runtime::SendSyncPtr\">SendSyncPtr</a>&lt;T&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wasmtime_runtime/struct.VMExternRef.html\" title=\"struct wasmtime_runtime::VMExternRef\">VMExternRef</a>"]],
"wyz":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtPointer.html\" title=\"struct wyz::fmt::FmtPointer\">FmtPointer</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Binary.html\" title=\"trait core::fmt::Binary\">Binary</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtBinary.html\" title=\"struct wyz::fmt::FmtBinary\">FmtBinary</a>&lt;T&gt;"],["impl&lt;M, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/comu/struct.Address.html\" title=\"struct wyz::comu::Address\">Address</a>&lt;M, T&gt;<span class=\"where fmt-newline\">where\n    M: <a class=\"trait\" href=\"wyz/comu/trait.Mutability.html\" title=\"trait wyz::comu::Mutability\">Mutability</a>,\n    T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.LowerHex.html\" title=\"trait core::fmt::LowerHex\">LowerHex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtLowerHex.html\" title=\"struct wyz::fmt::FmtLowerHex\">FmtLowerHex</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Octal.html\" title=\"trait core::fmt::Octal\">Octal</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtOctal.html\" title=\"struct wyz::fmt::FmtOctal\">FmtOctal</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.LowerExp.html\" title=\"trait core::fmt::LowerExp\">LowerExp</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtLowerExp.html\" title=\"struct wyz::fmt::FmtLowerExp\">FmtLowerExp</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.UpperHex.html\" title=\"trait core::fmt::UpperHex\">UpperHex</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtUpperHex.html\" title=\"struct wyz::fmt::FmtUpperHex\">FmtUpperHex</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Display.html\" title=\"trait core::fmt::Display\">Display</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtDisplay.html\" title=\"struct wyz::fmt::FmtDisplay\">FmtDisplay</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.UpperExp.html\" title=\"trait core::fmt::UpperExp\">UpperExp</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/fmt/trait.Pointer.html\" title=\"trait core::fmt::Pointer\">Pointer</a> for <a class=\"struct\" href=\"wyz/fmt/struct.FmtUpperExp.html\" title=\"struct wyz::fmt::FmtUpperExp\">FmtUpperExp</a>&lt;T&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()