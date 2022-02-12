interface NixObject {

	NixObject call(NixObject arg);

	default NixObject force() {
		return this.call(null);
	}

	default NixObject add(NixObject second) {
		return (arg) -> {
			NixObject.ensureLazy(arg);
			return NixInteger.create(((NixInteger) this.force()).value + ((NixInteger) second.force()).value);
		};
	}

	static void ensureLambda(NixObject arg) {
		if (arg == null) {
			throw new IllegalArgumentException("This is a lambda. Therefore you need to pass a parameter.");
		}
	}

	static void ensureLazy(NixObject arg) {
		if (arg != null) {
			throw new IllegalArgumentException("This is a lazy value and no lambda. Therefore you need to pass null.");
		}
	}
}