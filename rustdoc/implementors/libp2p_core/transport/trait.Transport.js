(function() {var implementors = {
"libp2p":[],
"libp2p_core":[],
"libp2p_dns":[["impl&lt;T, R&gt; <a class=\"trait\" href=\"libp2p_core/transport/trait.Transport.html\" title=\"trait libp2p_core::transport::Transport\">Transport</a> for <a class=\"struct\" href=\"libp2p_dns/struct.Transport.html\" title=\"struct libp2p_dns::Transport\">Transport</a>&lt;T, R&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"libp2p_core/transport/trait.Transport.html\" title=\"trait libp2p_core::transport::Transport\">Transport</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + 'static,\n    T::<a class=\"associatedtype\" href=\"libp2p_core/transport/trait.Transport.html#associatedtype.Error\" title=\"type libp2p_core::transport::Transport::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    T::<a class=\"associatedtype\" href=\"libp2p_core/transport/trait.Transport.html#associatedtype.Dial\" title=\"type libp2p_core::transport::Transport::Dial\">Dial</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + Resolver + 'static,</span>"]],
"libp2p_tcp":[["impl&lt;T&gt; <a class=\"trait\" href=\"libp2p_core/transport/trait.Transport.html\" title=\"trait libp2p_core::transport::Transport\">Transport</a> for <a class=\"struct\" href=\"libp2p_tcp/struct.Transport.html\" title=\"struct libp2p_tcp::Transport\">Transport</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: Provider + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    T::Listener: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,\n    T::Stream: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.73.0/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()