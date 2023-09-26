import { Icon as _Icon, IconProps } from "@iconify/react";

export const Icon = (props: IconProps) => {
  const Icon = _Icon;

  // @ts-ignore: ignore type error "cannot be used as a JSX component"
  return <Icon {...props} />;
};
