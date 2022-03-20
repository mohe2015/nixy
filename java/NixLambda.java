import java.util.ArrayDeque;
import java.util.Deque;
import java.util.Map;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

interface NixLambda extends NixValue {

	static NixLazy createFunction(NixLambda function) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return (NixLambda) (arg) -> {
					NixLambda.ensureLambda(arg);
					return function.call(arg);
				};
			}
		};
	}

	// maybe create a public call and an internal execute method so we could hide this in the call method
	static void ensureLambda(NixLazy arg) {
		if (arg == null) {
			throw new IllegalArgumentException("This is a lambda. Therefore you need to pass a parameter.");
		}
	}

	NixValue call(NixLazy arg);
}