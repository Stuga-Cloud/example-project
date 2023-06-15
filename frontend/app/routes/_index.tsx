import  type { V2_MetaFunction } from "@remix-run/node";
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

export type PageProps = {
  products: Product[],
  backendUrl: string,
}

export const loader = async (): Promise<PageProps> => {
  const backendUrl = process.env.BACKEND_URL as string;
  const data = await fetch(`${backendUrl}/v1/products`);
  const result = await data.json();
  return {products: result, backendUrl };
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
export const openModalAtom = atom(false);

function Page() {
  const { products, backendUrl } = useLoaderData<typeof loader>();
  const setProducts = useSetAtom(productAtom);
  setProducts(products);

  return (
    <>
      <Header />
      <ProductList />
      <ShoppingCart backendUrl={backendUrl} />
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

