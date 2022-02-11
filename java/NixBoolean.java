public class NixBoolean implements NixObject {

	boolean value;

	public NixBoolean(boolean value) {
		this.value = value;
	}

	@Override
	public NixObject call(NixObject arg) {
		return this;
	}

	@Override
	public String toString() {
		return "NixBoolean{" +
				"value=" + value +
				'}';
	}
}
