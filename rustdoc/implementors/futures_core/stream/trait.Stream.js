(function() {var implementors = {
"forest_filecoin":[["impl&lt;Inner, Accumulator, CollateFn, FinishFn, Collection&gt; Stream for <a class=\"struct\" href=\"forest_filecoin/utils/struct.TryCollate.html\" title=\"struct forest_filecoin::utils::TryCollate\">TryCollate</a>&lt;Inner, Accumulator, CollateFn, FinishFn, Collection&gt;<span class=\"where fmt-newline\">where\n    Inner: TryStream,\n    CollateFn: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"enum\" href=\"forest_filecoin/utils/enum.Collate.html\" title=\"enum forest_filecoin::utils::Collate\">Collate</a>&lt;Accumulator, Inner::Ok&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html\" title=\"enum core::ops::control_flow::ControlFlow\">ControlFlow</a>&lt;Collection, Accumulator&gt;,\n    FinishFn: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(Accumulator) -&gt; Collection,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()