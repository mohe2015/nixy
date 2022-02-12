interface NixObject {

	// TODO FIXME maybe split up into two subclasses lambda and lazy?
	// this *could* maybe help us with type-safety

	NixObject call(NixObject arg);

	default NixObject force() {
		return this.call(null);
	}

	default NixObject add(NixObject second) {
		NixObject.ensureLambda(second);
		return (arg) -> {
			NixObject.ensureLazy(arg);
			return NixInteger.create(((NixInteger) this.force()).value + ((NixInteger) second.force()).value).force();
		};
	}

	default NixObject eq(NixObject second) {
		NixObject.ensureLambda(second);
		return (arg) -> {
			NixObject.ensureLazy(arg);
			return NixBoolean.create(this.force().equals(second.force())).force();
		};
	}

	static NixObject createIf(NixObject condition, NixObject trueCase, NixObject falseCase) {
		return (arg) -> {
			NixObject.ensureLazy(arg);
			return ((NixBoolean)condition.force()).value ? trueCase.force() : falseCase.force();
		};
	}

	static NixObject createFunction(NixObject function) {
		return (a) -> {
			NixObject.ensureLazy(a);
			return (arg) -> {
				NixObject.ensureLambda(arg);
				return function.call(arg);
			};
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