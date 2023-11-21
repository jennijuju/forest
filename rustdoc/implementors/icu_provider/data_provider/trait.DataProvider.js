(function() {var implementors = {
"icu_provider":[],
"icu_provider_adapters":[["impl&lt;M&gt; <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt; for <a class=\"struct\" href=\"icu_provider_adapters/any_payload/struct.AnyPayloadProvider.html\" title=\"struct icu_provider_adapters::any_payload::AnyPayloadProvider\">AnyPayloadProvider</a><span class=\"where fmt-newline\">where\n    M: <a class=\"trait\" href=\"icu_provider/marker/trait.KeyedDataMarker.html\" title=\"trait icu_provider::marker::KeyedDataMarker\">KeyedDataMarker</a>,\n    for&lt;'a&gt; <a class=\"struct\" href=\"yoke/trait_hack/struct.YokeTraitHack.html\" title=\"struct yoke::trait_hack::YokeTraitHack\">YokeTraitHack</a>&lt;&lt;M::<a class=\"associatedtype\" href=\"icu_provider/marker/trait.DataMarker.html#associatedtype.Yokeable\" title=\"type icu_provider::marker::DataMarker::Yokeable\">Yokeable</a> as <a class=\"trait\" href=\"yoke/yokeable/trait.Yokeable.html\" title=\"trait yoke::yokeable::Yokeable\">Yokeable</a>&lt;'a&gt;&gt;::<a class=\"associatedtype\" href=\"yoke/yokeable/trait.Yokeable.html#associatedtype.Output\" title=\"type yoke::yokeable::Yokeable::Output\">Output</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    M::<a class=\"associatedtype\" href=\"icu_provider/marker/trait.DataMarker.html#associatedtype.Yokeable\" title=\"type icu_provider::marker::DataMarker::Yokeable\">Yokeable</a>: <a class=\"trait\" href=\"zerofrom/zero_from/trait.ZeroFrom.html\" title=\"trait zerofrom::zero_from::ZeroFrom\">ZeroFrom</a>&lt;'static, M::<a class=\"associatedtype\" href=\"icu_provider/marker/trait.DataMarker.html#associatedtype.Yokeable\" title=\"type icu_provider::marker::DataMarker::Yokeable\">Yokeable</a>&gt; + <a class=\"trait\" href=\"icu_provider/any/trait.MaybeSendSync.html\" title=\"trait icu_provider::any::MaybeSendSync\">MaybeSendSync</a>,</span>"],["impl&lt;M: <a class=\"trait\" href=\"icu_provider/marker/trait.KeyedDataMarker.html\" title=\"trait icu_provider::marker::KeyedDataMarker\">KeyedDataMarker</a>, P0: <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt;, P1: <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt;&gt; <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt; for <a class=\"enum\" href=\"icu_provider_adapters/either/enum.EitherProvider.html\" title=\"enum icu_provider_adapters::either::EitherProvider\">EitherProvider</a>&lt;P0, P1&gt;"],["impl&lt;M&gt; <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt; for <a class=\"struct\" href=\"icu_provider_adapters/empty/struct.EmptyDataProvider.html\" title=\"struct icu_provider_adapters::empty::EmptyDataProvider\">EmptyDataProvider</a><span class=\"where fmt-newline\">where\n    M: <a class=\"trait\" href=\"icu_provider/marker/trait.KeyedDataMarker.html\" title=\"trait icu_provider::marker::KeyedDataMarker\">KeyedDataMarker</a>,</span>"],["impl&lt;D, F, M&gt; <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt; for <a class=\"struct\" href=\"icu_provider_adapters/filter/struct.RequestFilterDataProvider.html\" title=\"struct icu_provider_adapters::filter::RequestFilterDataProvider\">RequestFilterDataProvider</a>&lt;D, F&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"struct\" href=\"icu_provider/request/struct.DataRequest.html\" title=\"struct icu_provider::request::DataRequest\">DataRequest</a>&lt;'_&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.bool.html\">bool</a>,\n    M: <a class=\"trait\" href=\"icu_provider/marker/trait.KeyedDataMarker.html\" title=\"trait icu_provider::marker::KeyedDataMarker\">KeyedDataMarker</a>,\n    D: <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt;,</span>"],["impl&lt;P, M&gt; <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt; for <a class=\"struct\" href=\"icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html\" title=\"struct icu_provider_adapters::fallback::LocaleFallbackProvider\">LocaleFallbackProvider</a>&lt;P&gt;<span class=\"where fmt-newline\">where\n    P: <a class=\"trait\" href=\"icu_provider/data_provider/trait.DataProvider.html\" title=\"trait icu_provider::data_provider::DataProvider\">DataProvider</a>&lt;M&gt;,\n    M: <a class=\"trait\" href=\"icu_provider/marker/trait.KeyedDataMarker.html\" title=\"trait icu_provider::marker::KeyedDataMarker\">KeyedDataMarker</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()