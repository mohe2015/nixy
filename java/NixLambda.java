interface NixLambda extends NixValue {

	static NixLazy createFunction(NixLambda function) {
		return () -> (NixLambda) (arg) -> {
			NixLambda.ensureLambda(arg);
			return function.call(arg);
		};
	}

	// maybe create a public call and an internal execute method so we could hide this in the call method
	static void ensureLambda(NixLazy arg) {
		if (arg == null) {
			throw new IllegalArgumentException("This is a lambda. Therefore you need to pass a parameter.");
		}
	}

	NixValue call(NixLazy arg);
}