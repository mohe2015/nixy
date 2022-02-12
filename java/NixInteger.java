import java.util.Objects;

public class NixInteger implements NixObject {

	int value;

	private NixInteger(int value) {
		this.value = value;
	}

	public static NixObject create(int value) {
		return (arg) -> {
			NixObject.ensureLazy(arg);
			return new NixInteger(value);
		};
	}

	@Override
	public NixObject call(NixObject arg) {
		throw new IllegalStateException("This is already a forced value");
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		NixInteger that = (NixInteger) o;
		return value == that.value;
	}

	@Override
	public int hashCode() {
		return Objects.hash(value);
	}

	@Override
	public String toString() {
		return "NixInteger{" +
				"value=" + value +
				'}';
	}
}
