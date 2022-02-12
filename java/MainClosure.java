public class MainClosure implements NixObject {

	public NixObject call(NixObject arg) {
		NixObject.ensureLambda(arg);
		return (arg2) -> {
			NixObject.ensureLazy(arg2);
			return ((NixBoolean)NixBoolean.create(true).force()).value ?
					NixInteger.create(1).add(arg).force() :
					NixInteger.create(2).add(call(arg)).force();
		};
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure().call(NixInteger.create(2)));
		System.out.println(new MainClosure().call(NixInteger.create(2)).force());
	}
}
