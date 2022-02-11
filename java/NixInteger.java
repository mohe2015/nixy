public class NixInteger implements NixObject {

	int value;

	public NixInteger(int value) {
		this.value = value;
	}

	public NixObject add(NixInteger nixInteger) {
		return new NixInteger(this.value + nixInteger.value);
	}

	@Override
	public NixObject call(NixObject arg) {
		return this;
	}

	@Override
	public String toString() {
		return "NixInteger{" +
				"value=" + value +
				'}';
	}
}
