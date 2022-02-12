public class NixBoolean implements NixObject {

	boolean value;

	private NixBoolean(boolean value) {
		this.value = value;
	}

	public static NixObject create(boolean value) {
		return (arg) -> {
			NixObject.ensureLazy(arg);
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
