import type { V2_MetaFunction } from "@remix-run/node";
import ShoppingCart from "~/components/cart";
import { Header } from "~/components/header";
import { ProductList, products } from "~/components/product-list";

export const meta: V2_MetaFunction = () => {
  return [
    { title: "New Remix App" },
    { name: "description", content: "Welcome to Remix!" },
  ];
};

export default function Index() {
  return (
    <>
      <Header />
      <ProductList products={products} />
      <ShoppingCart />
    </>
  );
}

