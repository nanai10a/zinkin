import { injectGlobal } from "@twind/core";

injectGlobal`
  .md-frame {
    & {
      @apply break-words;
    }

    & h1 {
      @apply mt-4 mt-2 text-2xl font-bold;
    }

    & h2 {
      @apply mt-4 mb-2 text-xl font-bold;
    }

    & h3 {
      @apply text-lg font-bold;
    }

    & h4, & h5, & h6 {
      @apply font-bold;
    }

    & a {
      @apply text-blue-500 underline;
    }

    & ul {
      @apply mt-1 mb-2 list-disc list-inside;
    }

    & ol {
      @apply mt-1 mb-2 list-decimal list-inside;
    }

    & li > ul > li,
    & li > ol > li {
      @apply ml-4;
    }

    & blockquote {
      @apply relative pl-4;

      &::before {
        content: "";
        @apply block absolute w-1 h-full bg-slate-300 rounded left-0;
      }
    }

    & code {
      @apply inline-block my-1 px-2 bg-slate-300 rounded font-mono;
    }

    & pre {
      @apply overflow-y-auto;
    }

    & img {
      @apply p-4 max-w-full h-auto;
    }
  }
`;

export const Markdown = ({ html: __html }: { html: string }) => (
  <div class="md-frame" dangerouslySetInnerHTML={{ __html }} />
);
