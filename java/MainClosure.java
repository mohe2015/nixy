import java.util.*;

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

	//Deque<List<String>> stackFrames = new ArrayDeque<>();

	public NixValue force3() {
		// let binding
		/*stackFrames.push(new ArrayList<>());
		stackFrames.peek().add("a");
		stackFrames.peek().add("b");*/
		currentVars.computeIfAbsent("a", (k) -> new ArrayDeque<>()).add(NixInteger.create(1));




		currentVars.get("a").pop();
		return null;
	}
}
