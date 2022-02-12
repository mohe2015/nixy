public class MainClosure implements NixLazy<NixLambda<NixInteger, NixInteger>> {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixLambda<NixInteger, NixInteger> force() {
		return NixLambda.createFunction((NixLambda<NixInteger, NixInteger>) a -> {
			return
					a.add(NixInteger.create(1)).force();
		}).force();
	}
}
