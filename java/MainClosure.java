import java.util.*;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class MainClosure extends NixLazyBase {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		return ((NixLazy) () -> {
			NixProxy x_ = new NixProxy();
			NixProxy y_ = new NixProxy();

			/* head */y_.proxy = x_.createCall().add(NixString.create("""
b""").add().createCall());
			x_.proxy = NixString.create("""
a""").add().createCall();

			/* body */
			return y_.createCall().add(NixString.create("""
c""").add().createCall()).force(); }).createCall().force();
	}

	IdentityHashMap<String, Deque<NixLazy>> currentVars = new IdentityHashMap<>();

	Deque<NixAttrset> scopes = new ArrayDeque<>();
	Deque<NixAttrset> withs = new ArrayDeque<>();

	public Optional<NixLazy> findVariable(String name) {
		Iterable<NixAttrset> scopesIterable = () -> scopes.descendingIterator();
		Stream<NixAttrset> scopesStream = StreamSupport.stream(scopesIterable.spliterator(), false);

		Iterable<NixAttrset> withsIterable = () -> withs.descendingIterator();
		Stream<NixAttrset> withsStream = StreamSupport.stream(withsIterable.spliterator(), false);

		 return Stream.concat(scopesStream, withsStream).flatMap(nixAttrset -> nixAttrset.value.entrySet().stream()).filter(entry -> entry.getKey().equals(name)).findFirst().map(Map.Entry::getValue);
	}

	public NixValue force3() {
		// let binding

		NixAttrset let = (NixAttrset) NixAttrset.create(new HashMap<>()).force();

		//currentVars.computeIfAbsent("a", (k) -> new ArrayDeque<>()).add(NixInteger.create(1));

		let.value.put("a", () -> scopes.descendingIterator());
		let.value.put("b", () -> NixInteger.create(5).force());


		currentVars.get("a").pop();
		return null;
	}
}
