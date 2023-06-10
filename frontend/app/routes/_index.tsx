import type { V2_MetaFunction } from "@remix-run/node";
import ShoppingCart from "~/components/cart";
import { Header } from "~/components/header";
import { ProductList, products } from "~/components/product-list";

export const meta: V2_MetaFunction = () => {
  return [
    { title: "Example project" },
    { name: "description", content: "description of example project" },
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

