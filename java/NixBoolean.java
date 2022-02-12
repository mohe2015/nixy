import java.util.Objects;

public class NixBoolean implements NixValue {

	boolean value;

	private NixBoolean(boolean value) {
		this.value = value;
	}

	public static NixLazy<NixBoolean> create(boolean value) {
		return () -> {
			return new NixBoolean(value);
		};
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
}
