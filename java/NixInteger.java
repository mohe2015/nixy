import java.util.Objects;

public class NixInteger implements NixValue {

	int value;

	private NixInteger(int value) {
		this.value = value;
	}

	public static NixLazy create(int value) {
		return () -> new NixInteger(value);
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
