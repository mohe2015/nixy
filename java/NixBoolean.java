import java.util.Objects;

public class NixBoolean implements NixValue, NixToString {

	boolean value;

	private NixBoolean(boolean value) {
		this.value = value;
	}

	public static NixLazy create(boolean value) {
		return () -> new NixBoolean(value);
	}

	@Override
	public NixValue call(NixLazy arg) {
		throw new IllegalStateException("can't call a boolean");
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		NixBoolean that = (NixBoolean) o;
		return value == that.value;
	}

	@Override
	public int hashCode() {
		return Objects.hash(value);
	}

	@Override
	public String toString() {
		return "NixBoolean{" +
				"value=" + value +
				'}';
	}

	@Override
	public NixString toNixString() {
		return (NixString) NixString.create(Boolean.toString(value)).force();
	}
}
