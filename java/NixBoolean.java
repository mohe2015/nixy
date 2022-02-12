public class NixBoolean implements NixObject {

	boolean value;

	private NixBoolean(boolean value) {
		this.value = value;
	}

	public static NixObject create(boolean value) {
		return (arg) -> {
			if (arg != null) {
				throw new IllegalArgumentException("This is a lazy value and no lambda. Therefore you need to pass null.");
			}
			return new NixBoolean(value);
		};
	}

	@Override
	public NixObject call(NixObject arg) {
		throw new IllegalStateException("This is already a forced value");
	}

	@Override
	public String toString() {
		return "NixBoolean{" +
				"value=" + value +
				'}';
	}
}
