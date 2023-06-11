import { redirect, type ActionArgs, type V2_MetaFunction } from "@remix-run/node";
import ShoppingCart from "~/components/cart";
import { Header } from "~/components/header";
import { ProductList } from "~/components/product-list";
import { useLoaderData } from "@remix-run/react";
import { Provider, atom, useSetAtom } from 'jotai';
import { atomWithStorage } from 'jotai/utils';

export const meta: V2_MetaFunction = () => {
  return [
    { title: "Example project" },
    { name: "description", content: "description of example project" },
  ];
};

export const loader = async (): Promise<Product[]> => {
  const backendUrl = process.env.BACKEND_URL as string;
  const data = await fetch(`${backendUrl}/v1/products`);
  const result = await data.json();
  return result;
}

export async function action({request}: ActionArgs) {
  const backendUrl = process.env.BACKEND_URL as string;
  const body = await request.formData();
  const cartItems = body.get('cart');
  const data = await fetch(`${backendUrl}/v1/products`, {
    method: "POST",
    body: cartItems,
    headers: {
      "Content-Type": "application/json"
    }
  });
  const response = await data.json();
  console.info(body, response);
  redirect("/");
  return null;
}


export type Product = {
  id: number,
  name: string,
  href: string,
  price: number,
  description: string,
  imageSrc: string,
  imageAlt: string,
}

export const productAtom = atom<Product[]>([]);
export const cartProductAtom = atomWithStorage<number[]>('cart', []);

function Page() {
  const loadedProducts = useLoaderData<typeof loader>();
  const setProducts = useSetAtom(productAtom);
  setProducts(loadedProducts);
  return (
    <>
      <Header />
      <ProductList />
      <ShoppingCart />
    </>
  );
}

export default function Index() {
  return (
    <Provider>
      <Page />
    </Provider>
  );
}
