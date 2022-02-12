public class NixInteger implements NixObject {

	int value;

	private NixInteger(int value) {
		this.value = value;
	}

	public static NixObject create(int value) {
		return (arg) -> {
			if (arg != null) {
				throw new IllegalArgumentException("This is a lazy value and no lambda. Therefore you need to pass null.");
			}
			return new NixInteger(value);
		};
	}

	public static NixObject add(NixObject first, NixObject second) {
		return (arg) -> {
			if (arg != null) {
				throw new IllegalArgumentException("This is a lazy value and no lambda. Therefore you need to pass null.");
			}
			return new NixInteger(((NixInteger) first.call(null)).value + ((NixInteger) second.call(null)).value);
		};
	}

	@Override
	public NixObject call(NixObject arg) {
		throw new IllegalStateException("This is already a forced value");
	}

	@Override
	public String toString() {
		return "NixInteger{" +
				"value=" + value +
				'}';
	}
}
