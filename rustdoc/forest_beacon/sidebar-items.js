window.SIDEBAR_ITEMS = {"constant":[["IGNORE_DRAND_VAR","Environmental Variable to ignore `Drand`. Lotus parallel is `LOTUS_IGNORE_DRAND`"]],"enum":[["DrandNetwork","Type of the `drand` network. In general only `mainnet` and its chain information should be considered stable."]],"mod":[["beacon_entries",""]],"struct":[["BeaconEntryJson","JSON beacon entry format. This matches the `drand` round JSON serialization API reference: https://drand.love/developer/http-api/#public-round."],["BeaconPoint","Contains height at which the beacon is activated, as well as the beacon itself."],["BeaconSchedule","Contains the vector of `BeaconPoint`, which are mappings of epoch to the `Randomness` beacons used."],["ChainInfo","Contains all the info about a `Drand` beacon chain. API reference: https://drand.love/developer/http-api/#info note: `groupHash` does not exist in docs currently, but is returned."],["DrandBeacon","`Drand` randomness beacon that can be used to generate randomness for the Filecoin chain. Primary use is to satisfy the [Beacon] trait."],["DrandConfig","Configuration used when initializing a `Drand` beacon."],["DrandPublic","Coefficients of the publicly available `Drand` keys. This is shared by all participants on the `Drand` network."],["MockBeacon","Mock beacon used for testing. Deterministic based on an interval."]],"trait":[["Beacon","Trait used as the interface to be able to retrieve bytes from a randomness beacon."]]};