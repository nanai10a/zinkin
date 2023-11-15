import { Icon as _Icon, IconProps, enableCache } from "@iconify/react";

enableCache("all");

export const Icon = (props: IconProps) => {
  const Icon = _Icon;

  // @ts-ignore: ignore type error "cannot be used as a JSX component"
  return <Icon {...props} />;
};
