import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public class NixArray implements NixValue {

	List<NixLazy> array;

	public NixArray(List<NixLazy> array) {
		this.array = array;
	}

	public static NixLazy create(List<NixLazy> value) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return new NixArray(value);
			}
		};
	}

	@Override
	public NixValue call(NixLazy arg) {
		throw new IllegalStateException("can't call an array");
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) return true;
		if (o == null || getClass() != o.getClass()) return false;
		NixArray nixArray = (NixArray) o;
		return Objects.equals(array, nixArray.array);
	}

	@Override
	public int hashCode() {
		return Objects.hash(array);
	}

	@Override
	public String toString() {
		return "NixArray{" +
				"array=" + array +
				'}';
	}
}
