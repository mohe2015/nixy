interface NixObject {

	NixObject call(NixObject arg);

	default NixObject add(NixObject second) {
		return (arg) -> {
			if (arg != null) {
				throw new IllegalArgumentException("This is a lazy value and no lambda. Therefore you need to pass null.");
			}
			return NixInteger.create(((NixInteger) this.call(null)).value + ((NixInteger) second.call(null)).value);
		};
	}
}