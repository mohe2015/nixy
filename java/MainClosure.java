import java.util.*;

public class MainClosure extends NixLazyBase {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	IdentityHashMap<String, Deque<NixLazy>> currentVars = new IdentityHashMap<>();

	//Deque<List<String>> stackFrames = new ArrayDeque<>();

	public NixValue force() {
		// let binding
		/*stackFrames.push(new ArrayList<>());
		stackFrames.peek().add("a");
		stackFrames.peek().add("b");*/
		currentVars.computeIfAbsent("a", (k) -> new ArrayDeque<>()).add(NixInteger.create(1));



		currentVars.get("a").pop();
		return null;
	}
}
