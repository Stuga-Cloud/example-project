import type { V2_MetaFunction } from "@remix-run/node";
import ShoppingCart from "~/components/cart";
import { Header } from "~/components/header";
import { Product, ProductList, } from "~/components/product-list";
import { useLoaderData } from "@remix-run/react";

export const loader = async (): Promise<Product[]> => {
  const backendUrl =process.env.BACKEND_URL as string;
  const data = await fetch(`${backendUrl}/v1/products`);
  const result = await data.json();
  return result;
}

export const meta: V2_MetaFunction = () => {
  return [
    { title: "Example project" },
    { name: "description", content: "description of example project" },
  ];
};

export default function Index() {
  const products = useLoaderData<typeof loader>();
  return (
    <>
      <Header />
      <ProductList products={products} />
      <ShoppingCart />
    </>
  );
}

