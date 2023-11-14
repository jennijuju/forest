(function() {var implementors = {
"storage_proofs_core":[["impl&lt;Tree: <a class=\"trait\" href=\"storage_proofs_core/merkle/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::MerkleTreeTrait\">MerkleTreeTrait</a>&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for <a class=\"struct\" href=\"storage_proofs_core/gadgets/por/struct.PoRCircuit.html\" title=\"struct storage_proofs_core::gadgets::por::PoRCircuit\">PoRCircuit</a>&lt;Tree&gt;"]],
"storage_proofs_porep":[["impl&lt;'a, Tree: <a class=\"trait\" href=\"storage_proofs_core/merkle/tree/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::tree::MerkleTreeTrait\">MerkleTreeTrait</a>, G: <a class=\"trait\" href=\"filecoin_hashers/types/trait.Hasher.html\" title=\"trait filecoin_hashers::types::Hasher\">Hasher</a>&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for <a class=\"struct\" href=\"storage_proofs_porep/stacked/struct.StackedCircuit.html\" title=\"struct storage_proofs_porep::stacked::StackedCircuit\">StackedCircuit</a>&lt;'a, Tree, G&gt;"]],
"storage_proofs_post":[["impl&lt;Tree: 'static + <a class=\"trait\" href=\"storage_proofs_core/merkle/tree/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::tree::MerkleTreeTrait\">MerkleTreeTrait</a>&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for <a class=\"struct\" href=\"storage_proofs_post/election/struct.ElectionPoStCircuit.html\" title=\"struct storage_proofs_post::election::ElectionPoStCircuit\">ElectionPoStCircuit</a>&lt;Tree&gt;"],["impl&lt;Tree: 'static + <a class=\"trait\" href=\"storage_proofs_core/merkle/tree/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::tree::MerkleTreeTrait\">MerkleTreeTrait</a>&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for <a class=\"struct\" href=\"storage_proofs_post/rational/struct.RationalPoStCircuit.html\" title=\"struct storage_proofs_post::rational::RationalPoStCircuit\">RationalPoStCircuit</a>&lt;Tree&gt;"],["impl&lt;Tree: 'static + <a class=\"trait\" href=\"storage_proofs_core/merkle/tree/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::tree::MerkleTreeTrait\">MerkleTreeTrait</a>&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for <a class=\"struct\" href=\"storage_proofs_post/fallback/struct.FallbackPoStCircuit.html\" title=\"struct storage_proofs_post::fallback::FallbackPoStCircuit\">FallbackPoStCircuit</a>&lt;Tree&gt;"],["impl&lt;Tree: 'static + <a class=\"trait\" href=\"storage_proofs_core/merkle/tree/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::tree::MerkleTreeTrait\">MerkleTreeTrait</a>&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for &amp;<a class=\"struct\" href=\"storage_proofs_post/fallback/struct.Sector.html\" title=\"struct storage_proofs_post::fallback::Sector\">Sector</a>&lt;Tree&gt;"]],
"storage_proofs_update":[["impl&lt;TreeR&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for <a class=\"struct\" href=\"storage_proofs_update/poseidon/circuit/struct.EmptySectorUpdateCircuit.html\" title=\"struct storage_proofs_update::poseidon::circuit::EmptySectorUpdateCircuit\">EmptySectorUpdateCircuit</a>&lt;TreeR&gt;<span class=\"where fmt-newline\">where\n    TreeR: <a class=\"trait\" href=\"storage_proofs_core/merkle/tree/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::tree::MerkleTreeTrait\">MerkleTreeTrait</a>&lt;Hasher = <a class=\"type\" href=\"storage_proofs_update/constants/type.TreeRHasher.html\" title=\"type storage_proofs_update::constants::TreeRHasher\">TreeRHasher</a>&gt;,</span>"],["impl&lt;TreeR&gt; <a class=\"trait\" href=\"bellpepper_core/constraint_system/trait.Circuit.html\" title=\"trait bellpepper_core::constraint_system::Circuit\">Circuit</a>&lt;<a class=\"struct\" href=\"blstrs/scalar/struct.Scalar.html\" title=\"struct blstrs::scalar::Scalar\">Scalar</a>&gt; for <a class=\"struct\" href=\"storage_proofs_update/circuit/struct.EmptySectorUpdateCircuit.html\" title=\"struct storage_proofs_update::circuit::EmptySectorUpdateCircuit\">EmptySectorUpdateCircuit</a>&lt;TreeR&gt;<span class=\"where fmt-newline\">where\n    TreeR: <a class=\"trait\" href=\"storage_proofs_core/merkle/tree/trait.MerkleTreeTrait.html\" title=\"trait storage_proofs_core::merkle::tree::MerkleTreeTrait\">MerkleTreeTrait</a>&lt;Hasher = <a class=\"type\" href=\"storage_proofs_update/constants/type.TreeRHasher.html\" title=\"type storage_proofs_update::constants::TreeRHasher\">TreeRHasher</a>&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()