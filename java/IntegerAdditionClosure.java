public class IntegerAdditionClosure implements NixObject {

	public NixObject call(NixObject first) {
		return (second) -> {
			NixInteger firstInt = (NixInteger) first;
			NixInteger secondInt = (NixInteger) second;
			return firstInt.add(secondInt);
		};
	}
}
