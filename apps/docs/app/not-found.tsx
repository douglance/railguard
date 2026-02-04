import Link from 'next/link';

export default function NotFound() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center">
      <h1 className="text-4xl font-bold">404</h1>
      <p className="mt-4 text-fd-muted-foreground">Page not found</p>
      <Link
        href="/"
        className="mt-6 text-fd-primary hover:underline"
      >
        Go home
      </Link>
    </div>
  );
}
