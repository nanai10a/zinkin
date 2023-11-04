import type { Plugin, OutputAsset } from "rollup";

import path from "node:path";

import ve from "@vanilla-extract/integration";
import swc from "@swc/core";

import { JSDOM } from "jsdom";

type Options = {
  basedir: string;
  identOption?: ve.IdentifierOption;
};

const VanillaExtractPlugin = (opts: Options): Plugin => {
  const { basedir, identOption = "short" } = opts;

  const csses = new Set<string>();

  return {
    name: "vanilla-extract",

    async transform(_code, id) {
      // targets are only likes `.ts.css`
      if (!ve.cssFileFilter.test(id)) {
        return null;
      }

      // built source; outs dirty js
      const { source, watchFiles } = await ve.compile({
        filePath: id,
        identOption,
      });

      // register as having to watch
      watchFiles.forEach((file) => this.addWatchFile(file));

      // outs js that only exports classes
      const code = await ve.processVanillaFile({
        source,
        filePath: id,
        identOption,
      });

      // parse js, into ast
      const { body } = await swc.parse(code, {
        syntax: "ecmascript",
        comments: false,
      });

      // only interest to import-statement
      const isImport = (item: swc.ModuleItem): item is swc.ImportDeclaration =>
        item.type === "ImportDeclaration";

      // extract imports that point to vanilla-extract's virtual css files
      const cssImports = body
        .filter(isImport)
        .filter((stmt) => ve.virtualCssFileFilter.test(stmt.source.value));

      for (const stmt of cssImports) {
        // get actual css
        const { fileName, source } = await ve.getSourceFromVirtualCssFile(
          stmt.source.value,
        );

        // omit basedir
        const filePath = path.relative(
          basedir,
          fileName.replace(".css.ts", ""),
        );

        // emit css as asset
        this.emitFile({
          type: "asset",
          fileName: filePath,
          source,
        });

        // ...and use later, to inject into html
        csses.add(filePath);
      }

      // for remove css imports; browser won't allow them
      const replaceToVoid = (code: string, span: swc.Span) => {
        const raw = code.slice(span.start - 1, span.end);
        const voidy = raw.replace(/./g, " ");

        return code.replace(raw, voidy);
      };

      // finally, outs js that been removed css imports
      return cssImports.map(({ span }) => span).reduce(replaceToVoid, code);
    },

    async generateBundle(_opts, bundle, _isWrite) {
      // extract htmls for inject css
      const htmls = Object.keys(bundle)
        .map((k) => bundle[k])
        .filter((info): info is OutputAsset => info.type === "asset")
        .filter((info) => info.fileName.endsWith(".html"));

      /// DOUBT: does bundle include many htmls?
      if (htmls.length !== 1) {
        throw new Error();
      }

      // ...now it's assumed as only one html
      // because when inject csses into all htmls, they potentially have unnecessary links
      const html = htmls[0];

      // make instance of virtual dom
      const dom = new JSDOM(html.source);
      const head = dom.window.document.getElementsByTagName("head")[0];

      // inject `<link>` into `<head>`
      Array.from(csses)
        .filter((name) => Object.keys(bundle).includes(name))
        .forEach((href) => {
          const link = dom.window.document.createElement("link");

          link.rel = "stylesheet";
          link.href = href;

          head.appendChild(link);
        });

      // apply changes
      html.source = dom.serialize();
    },
  };
};

export default VanillaExtractPlugin;
