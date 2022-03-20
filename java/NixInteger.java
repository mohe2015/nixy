import java.util.Objects;

public class NixInteger implements NixValue, NixNumber, NixToString {

	int value;

	private NixInteger(int value) {
		this.value = value;
	}

	public static NixLazy create(int value) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return new NixInteger(value);
			}
		};
	}

	@Override
	public NixValue call(NixLazy arg) {
		throw new IllegalStateException("can't call an integer");
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

	@Override
	public NixFloat toNixFloat() {
		return (NixFloat) NixFloat.create(this.value).force();
	}

	@Override
	public NixString toNixString() {
		return (NixString) NixString.create(Integer.toString(value)).force();
	}
}
